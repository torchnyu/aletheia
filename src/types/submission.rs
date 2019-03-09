use crate::schema::*;
use crate::types::{Project, User, UserResponse};
use diesel::{self, AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "submissions"]
#[belongs_to(User)]
#[belongs_to(UserResponse, foreign_key = "user_id")]
#[belongs_to(Project)]
pub struct Submission {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "submissions"]
pub struct SubmissionInsert {
    pub user_id: i32,
    pub project_id: i32,
}

impl SubmissionInsert {
    pub fn from_submission(submission: Submission) -> SubmissionInsert {
        SubmissionInsert {
            user_id: submission.user_id,
            project_id: submission.project_id,
        }
    }
}
