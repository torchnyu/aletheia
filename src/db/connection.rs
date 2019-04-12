use diesel::pg::PgConnection;
use crate::db::sql_types::*;

#[database("postgres")]
pub struct Connection(PgConnection);

pub struct RequestContext {
    pub conn: Connection,
}

impl RequestContext {

    pub fn from_connection(conn: Connection) -> Self {
        Self {
            conn
        }
    }

    pub fn database(&self, action: ActionType, modifier: ActionModifier) -> DatabaseContext {
        return DatabaseContext::from(&self.conn, action, modifier);
    }
}

/// Context struct that contains information about
/// how we're using the database
pub struct DatabaseContext<'a> {

    // Eventually make this private
    pub conn: &'a Connection,
    action: ActionType,
    modifier: ActionModifier
}

impl<'a> DatabaseContext<'a> {

    fn from(conn: &'a Connection, action: ActionType, modifier: ActionModifier) -> Self {
        Self {
            conn, action, modifier
        }
    }
}

