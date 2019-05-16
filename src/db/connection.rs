use crate::db::sql_types::*;
use crate::types::Token;
use diesel::pg::PgConnection;
use rocket::fairing::{AdHoc, Fairing};
use rocket::logger::error;
use rocket_contrib::databases::r2d2::{Pool, PooledConnection};
use rocket_contrib::databases::{database_config, Poolable};

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
    pub fn database_context(
        &self,
        token: Option<Token>,
        action: ActionType,
        modifier: ActionModifier,
    ) -> DatabaseContext {
        return DatabaseContext::from(&self.conn, token, action, modifier);
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
    pub token: Token,
    pub action: ActionType,
    pub modifier: ActionModifier,
}

impl<'a> DatabaseContext<'a> {
    /// Create a DatabaseContext Struct from a connection and modifier information
    fn from(
        conn: &'a PgConnection,
        token: Option<Token>,
        action: ActionType,
        modifier: ActionModifier,
    ) -> Self {
        let token = match token {
            Some(token) => token,
            None => Token::new_invalid(),
        };
        Self {
            conn,
            token,
            action,
            modifier,
        }
    }
}
