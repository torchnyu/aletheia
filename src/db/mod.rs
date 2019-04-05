use diesel::pg::PgConnection;
/*
use diesel::r2d2::ConnectionManager;
use diesel::Connection as DieselConnection;
use r2d2;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
use std::ops::Deref;
use rocket_contrib::databases::diesel;
*/

#[database("DATABASE_URL")]
pub struct Connection(PgConnection);

// pub struct Connection(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/*
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url[..]);
    r2d2::Pool::new(manager).expect(&format!("Error connection to {}", &database_url[..]))
}

pub struct Connection(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
        let pool = match <State<Pool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
*/
