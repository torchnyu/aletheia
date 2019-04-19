use crate::db::models::{ProjectInsert, ProjectRequest};
use crate::db::schema::{events, projects};
use crate::utils::Result;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sql_types;
use phf::phf_map;
use slug::slugify;

#[derive(Debug, Fail, PartialEq)]
pub enum ValidateError {
    #[fail(display = "Project name cannot end in '-': {}", project_name)]
    DashSuffix { project_name: String },
    #[fail(display = "Cannot name project {}, is a reserved word", project_name)]
    ReservedWord { project_name: String },
}

static RESERVED_WORDS: phf::Map<&'static str, ()> = phf_map! {
    "admin" => (),
    "login" => (),
    "submit" => (),
    "delete" => (),
};

sql_function!(fn nextval(a: sql_types::Text) -> sql_types::BigInt);

pub fn call(project: ProjectRequest, conn: &diesel::PgConnection) -> Result<ProjectInsert> {
    // This screws up our slug naming convention
    if project.name.ends_with("-") {
        Err(ValidateError::DashSuffix {
            project_name: project.name.clone(),
        })?;
    }
    let lowercase_name = project.name.to_lowercase();
    if let Some(_) = RESERVED_WORDS.get(&lowercase_name[..]) {
        Err(ValidateError::ReservedWord {
            project_name: project.name.clone(),
        })?
    }

    // Claims the next id and returns it.
    let new_id: Vec<i64> = select(nextval("projects_id_seq")).load(conn)?;
    let mut slug = slugify(&project.name);
    let does_slug_exist =
        select(exists(projects::table.filter(projects::slug.eq(&slug)))).get_result(conn)?;
    // If slug exists, append the id number. Since names cannot end in
    // a dash, we know that this has to be unique.
    if does_slug_exist {
        slug = format!("{}-{}", slug, new_id[0]);
    };
    let event_id = events::table
        .filter(events::slug.eq(project.event_slug))
        .select(events::id)
        .first(conn)?;

    Ok(ProjectInsert {
        // This will bite me in the ass when we run out of 32
        // bit integers, but that'll be the day
        id: new_id[0] as i32,
        name: project.name,
        repository_url: project.repository_url,
        description: project.description,
        slug,
        event_id,
    })
}
