use diesel::pg::PgConnection;

#[database("postgres")]
pub struct Connection(PgConnection);

