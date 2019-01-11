#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::*;
use diesel;
use diesel::{AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "projects"]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub repository_id: i32,
}
