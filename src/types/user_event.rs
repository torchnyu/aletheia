use super::{Event, RawUser, User};
use crate::schema::*;
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "user_events"]
pub struct UserEventInsert {
    pub user_id: i32,
    pub event_id: i32,
}

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "user_events"]
#[belongs_to(RawUser, foreign_key = "user_id")]
#[belongs_to(User)]
#[belongs_to(Event)]
pub struct UserEvent {
    id: i32,
    user_id: i32,
    event_id: i32,
}
