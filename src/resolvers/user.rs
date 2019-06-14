use crate::db::connection::DatabaseContext;
use crate::db::models::{LoginRequest, RawUser, Role, User, UserInsert, UserRequest, UserRole};
use crate::db::schema::{roles, user_roles, users};
use crate::utils::{AletheiaError, Result};
use argonautica::input::Salt;
use argonautica::{Hasher, Verifier};
use diesel::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use rocket_contrib::databases::diesel;
use std::env;

pub fn all(db: &DatabaseContext) -> Result<Vec<User>> {
    Ok(users::table
        .select((users::id, users::display_name, users::email))
        .load::<User>(db.conn)?)
}

pub fn create(user: UserRequest, db: &DatabaseContext) -> Result<User> {
    let user_exists = match &user.display_name {
        Some(display_name) => select(exists(
            users::table
                .filter(users::email.eq(&(user.email)))
                .or_filter(users::display_name.eq(&(display_name))),
        ))
        .get_result(db.conn)?,
        None => select(exists(users::table.filter(users::email.eq(&(user.email)))))
            .get_result(db.conn)?,
    };
    if user_exists {
        return Err(AletheiaError::UserAlreadyExists {
            email: user.email.clone(),
        })?;
    }
    let user = UserInsert::from_request(user)?;
    let user = diesel::insert_into(users::table)
        .values(&user)
        .get_result(db.conn)?;
    Ok(User::from_raw_user(user))
}

pub fn login(credentials: &LoginRequest, db: &DatabaseContext) -> Result<User> {
    let user: RawUser = users::table
        .filter(users::email.eq(&(credentials.email)))
        .first(db.conn)?;
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

impl User {
    pub fn roles(&self, db: &DatabaseContext) -> Vec<Role> {
        let role_ids = UserRole::belonging_to(self).select(user_roles::role_id);
        roles::table
            .filter(roles::id.eq(any(role_ids)))
            .load::<Role>(db.conn)
            .expect("Could not load contributors")
    }
}

impl UserInsert {
    pub fn from_request(request: UserRequest) -> Result<UserInsert> {
        let mut hasher = Hasher::default();
        let salt_length = env::var("SALT_LENGTH")?;
        let salt = Salt::random(salt_length.parse::<u32>()?);
        let salt = salt.to_str()?;
        let password_digest = hasher
            .with_password(request.password)
            .with_secret_key(env::var("SECRET_KEY")?)
            .with_salt(salt)
            .hash()?;
        Ok(UserInsert {
            display_name: request.display_name,
            email: request.email,
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
