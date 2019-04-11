#[allow(unused_imports)]
pub mod schema;
pub mod sql_types;

use diesel::pg::PgConnection;

#[database("postgres")]
pub struct Connection(PgConnection);