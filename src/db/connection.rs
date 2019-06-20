use crate::db::sql_types::ActionType;
use diesel::pg::PgConnection;
use rocket::fairing::Fairing;
use rocket_contrib::databases::r2d2::{Pool, PooledConnection};
use rocket_contrib::databases::Poolable;

#[derive(Debug, Fail)]
pub enum AuthError {
    #[fail(display = "You are not authorized to {:?} {}", action, resource)]
    NoPermission {
        action: ActionType,
        resource: String,
    },
    #[fail(display = "No associated user to authenticate with")]
    NoUser,
}

type Connection = PooledConnection<<PgConnection as Poolable>::Manager>;

/// The request guard type.
pub struct RequestContext {
    pub conn: Connection,
}

/// The pool type.
pub struct ConnectionPool(Pool<<PgConnection as Poolable>::Manager>);

impl RequestContext {
    /// Returns a fairing that initializes the associated database
    /// connection pool.
    pub fn fairing() -> impl Fairing {
        use rocket::fairing::AdHoc;
        use rocket::logger::error;
        use rocket_contrib::databases::database_config;

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

impl From<Connection> for RequestContext {
    fn from(conn: Connection) -> Self {
        Self { conn }
    }
}
