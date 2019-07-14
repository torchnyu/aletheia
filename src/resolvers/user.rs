use crate::db::models::{
    LoginRequest, Medium, PasswordResetRequest, RawUser, Role, User, UserInsert, UserRequest,
    UserRole,
};
use crate::db::schema::{media, password_reset_requests, roles, user_roles, users};
use crate::services;
use crate::utils::{AletheiaError, Result};
use argonautica::Hasher;
use argonautica::Verifier;
use chrono::Utc;
use diesel::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use rocket_contrib::databases::diesel;
use std::env;

pub fn all(conn: &PgConnection) -> Result<Vec<User>> {
    Ok(users::table
        .select((users::id, users::display_name, users::email))
        .load::<User>(conn)?)
}

pub fn create(user: UserRequest, conn: &PgConnection) -> Result<User> {
    let user_exists = match &user.display_name {
        Some(display_name) => select(exists(
            users::table
                .filter(users::email.eq(&(user.email)))
                .or_filter(users::display_name.eq(&(display_name))),
        ))
        .get_result(conn)?,
        None => {
            select(exists(users::table.filter(users::email.eq(&(user.email))))).get_result(conn)?
        }
    };
    if user_exists {
        return Err(AletheiaError::UserAlreadyExists {
            email: user.email.clone(),
        })?;
    }
    let user = UserInsert::from_request(user)?;
    let user = diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)?;
    Ok(User::from_raw_user(user))
}

pub fn login(credentials: &LoginRequest, conn: &PgConnection) -> Result<User> {
    let user: RawUser = users::table
        .filter(users::email.eq(&(credentials.email)))
        .first(conn)?;
    if user.validate_credentials(credentials)? {
        Ok(User::from_raw_user(user))
    } else {
        Err(AletheiaError::NoUserError {
            email: credentials.email.clone(),
        })?
    }
}

pub fn get_by_email(email: &str, conn: &PgConnection) -> Result<User> {
    let user: RawUser = users::table.filter(users::email.eq(email)).first(conn)?;
    Ok(User::from_raw_user(user))
}

pub fn send_reset_email(email: &str, domain: &str, conn: &diesel::PgConnection) -> Result<()> {
    // Want to get the user but also not error if None
    let user = match users::table
        .filter(users::email.eq(email))
        .first::<RawUser>(conn)
        .optional()?
    {
        None => {
            // Sends email saying "sorry there's no corresponding user"
            services::user::send_no_user_reset_email()?;
            return Ok(());
        }
        Some(user) => User::from_raw_user(user),
    };

    let reset_request = services::user::send_reset_password_email(&user, domain)?;
    diesel::insert_into(password_reset_requests::table)
        .values(reset_request)
        .execute(conn)?;
    Ok(())
}

static ONE_DAY: i64 = 60 * 60 * 24;

pub fn reset_password(
    email: &str,
    password: &str,
    key: &str,
    conn: &diesel::PgConnection,
) -> Result<User> {
    let user = get_by_email(email, conn)?;
    let mut hasher = Hasher::default();
    // Hash the random bytes
    let id = hasher
        .with_password(key)
        .with_salt(env::var("KEY_SALT")?)
        .with_secret_key(env::var("SECRET_KEY")?)
        .hash()?;
    let reset_request = password_reset_requests::table
        .filter(password_reset_requests::user_id.eq(user.id))
        .filter(password_reset_requests::id.eq(id))
        .first::<PasswordResetRequest>(conn)?;

    if Utc::now().timestamp() - reset_request.created_at.timestamp() > ONE_DAY {
        Err(AletheiaError::ExpiredResetKey.into())
    } else {
        let new_password_digest = services::user::hash_password(&password)?;
        diesel::delete(&reset_request).execute(conn)?;
        diesel::update(&user)
            .set(users::password_digest.eq(new_password_digest))
            .execute(conn)?;
        Ok(user)
    }
}

impl User {
    pub fn roles(&self, conn: &PgConnection) -> Vec<Role> {
        let role_ids = UserRole::belonging_to(self).select(user_roles::role_id);
        roles::table
            .filter(roles::id.eq(any(role_ids)))
            .load::<Role>(conn)
            .expect("Could not load contributors")
    }

    pub fn profile_picture(&self, conn: &diesel::PgConnection) -> Option<Medium> {
        media::table
            .filter(media::user_id.eq(self.id))
            .first::<Medium>(conn)
            .optional()
            .expect("Could not load profile picture")
    }
}

impl UserInsert {
    pub fn from_request(request: UserRequest) -> Result<UserInsert> {
        let password_digest = services::user::hash_password(&request.password)?;
        Ok(UserInsert {
            display_name: request.display_name,
            email: request.email.to_ascii_lowercase(),
            password_digest,
        })
    }
}

impl RawUser {
    pub fn validate_credentials(self: &RawUser, creds: &LoginRequest) -> Result<bool> {
        let mut verifier = Verifier::default();
        Ok(verifier
            .with_hash(self.password_digest.clone())
            .with_password(creds.password.clone())
            .with_secret_key(env::var("SECRET_KEY")?)
            .verify()?)
    }
}
