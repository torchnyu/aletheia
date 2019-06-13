use crate::db::models::Permission;
use crate::db::models::User;
use crate::db::sql_types::*;
use crate::types::Token;
use crate::utils::*;
use diesel::pg::PgConnection;
use rocket::fairing::{AdHoc, Fairing};
use rocket::logger::error;
use rocket_contrib::databases::r2d2::{Pool, PooledConnection};
use rocket_contrib::databases::{database_config, Poolable};

pub enum AuthState {
    Valid {
        user: User,
        permissions: Vec<Permission>,
    },
    Invalid {
        user: User,
    },
    Anon,
}

#[derive(Debug, Fail)]
pub enum AuthError {
    #[fail(display = "You are not authorized to {:?} {}", action, resource)]
    NoPermission {
        action: ActionType,
        resource: String,
    },
    #[fail(display = "No associated user to autheticate with")]
    NoUser,
}

type Connection = PooledConnection<<PgConnection as Poolable>::Manager>;

/// The request guard type.
pub struct RequestContext {
    conn: Connection,
}

/// The pool type.
pub struct ConnectionPool(Pool<<PgConnection as Poolable>::Manager>);

impl From<Connection> for RequestContext {
    fn from(conn: Connection) -> Self {
        Self { conn }
    }
}
impl RequestContext {
    /// Get a DatabaseContext from this struct that can be used to connect
    /// to the db
    pub fn database_context<'a>(
        &'a self,
        resource: &'a str,
        token: Option<Token>,
        action: ActionType,
        modifier: ActionModifier,
    ) -> Result<DatabaseContext<'a>> {
        DatabaseContext::from(&self.conn, resource, token, action, modifier)
    }

    pub fn db_context_for_anon_user(
        &self,
        action: ActionType,
        modifier: ActionModifier,
    ) -> DatabaseContext {
        DatabaseContext::from(&self.conn, &"none", None, action, modifier)
            .expect("Error is unreachable")
    }

    /// Returns a fairing that initializes the associated database
    /// connection pool.
    pub fn fairing() -> impl Fairing {
        AdHoc::on_attach("\'postgres\' Database Pool", |rocket| {
            let config = match database_config("postgres", rocket.config()) {
                Ok(cfg) => cfg,
                Err(config_error) => {
                    error(&format!(
                        "Database configuration failure (postgres): {:?}",
                        config_error
                    ));
                    return Err(rocket);
                }
            };
            let pool = match PgConnection::pool(config) {
                Ok(p) => p,
                Err(pool_error) => {
                    error(&format!(
                        "Failed to initialize pool for 'postgres': {:?}",
                        pool_error
                    ));
                    return Err(rocket);
                }
            };
            Ok(rocket.manage(ConnectionPool(pool)))
        })
    }
    /// Retrieves a connection of type `Self` from the `rocket`
    /// instance. Returns `Some` as long as `Self::fairing()` has been
    /// attached and there is at least one connection in the pool.
    pub fn get_one(rocket: &::rocket::Rocket) -> Option<Self> {
        rocket
            .state::<ConnectionPool>()
            .and_then(|pool| pool.0.get().ok())
            .map(|conn| RequestContext::from(conn))
    }
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for RequestContext {
    type Error = ();
    fn from_request(
        request: &'a rocket::request::Request<'r>,
    ) -> rocket::request::Outcome<Self, ()> {
        use rocket::{http::Status, Outcome};
        let pool = request.guard::<::rocket::State<ConnectionPool>>()?;
        match pool.0.get() {
            Ok(conn) => Outcome::Success(RequestContext::from(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

/// Context struct that contains information about
/// how we're using the database
pub struct DatabaseContext<'a> {
    // Eventually make this private
    pub conn: &'a PgConnection,
    pub resource: &'a str,
    pub auth: AuthState,
    pub action: ActionType,
    pub modifier: ActionModifier,
}

impl<'a> DatabaseContext<'a> {
    fn try_get_permissions(
        conn: &'a PgConnection,
        resource: &'a str,
        token: Option<Token>,
        action: ActionType,
        modifier: ActionModifier,
    ) -> Result<AuthState> {
        use crate::resolvers::permission;
        use crate::resolvers::user;
        if let Some(token) = token {
            let user = user::get_by_email(&token.uid, conn)?;
            let permissions = permission::get_permission(conn, &user, resource, action, modifier)?;
            if permissions.is_empty() {
                Ok(AuthState::Invalid { user })
            } else {
                Ok(AuthState::Valid { user, permissions })
            }
        } else {
            Ok(AuthState::Anon)
        }
    }
    /// Create a DatabaseContext Struct from a connection and modifier information
    fn from(
        conn: &'a PgConnection,
        resource: &'a str,
        token: Option<Token>,
        action: ActionType,
        modifier: ActionModifier,
    ) -> Result<Self> {
        let auth = Self::try_get_permissions(conn, resource, token, action, modifier)?;
        Ok(Self {
            conn,
            auth,
            resource,
            action,
            modifier,
        })
    }

    pub fn get_user(&self) -> Result<&User> {
        match &self.auth {
            &AuthState::Anon => Err(AuthError::NoUser)?,
            &AuthState::Valid {
                ref user,
                permissions: _,
            } => Ok(user),
            &AuthState::Invalid { ref user } => Ok(user),
        }
    }

    pub fn get_permissions(&self) -> Result<&Vec<Permission>> {
        match &self.auth {
            &AuthState::Anon => Err(AuthError::NoUser)?,
            &AuthState::Valid {
                user: _,
                ref permissions,
            } => Ok(permissions),
            &AuthState::Invalid { .. } => Err(AuthError::NoPermission {
                action: self.action,
                resource: self.resource.to_string(),
            })?,
        }
    }

    pub fn get_resource(&self) -> &str {
        &self.resource
    }

    pub fn get_action(&self) -> ActionType {
        self.action
    }

    pub fn get_modifier(&self) -> ActionModifier {
        self.modifier
    }
}
