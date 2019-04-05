use diesel::pg::PgConnection;

#[database("DATABASE_URL")]
pub struct Connection(PgConnection);

