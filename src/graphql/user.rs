use super::RequestContext;
use crate::db::schema::{projects, submissions};
use crate::db::sql_types::{ActionModifier, ActionType};
use crate::types::{Medium, Project, Submission, User};
use diesel::pg::expression::dsl::any;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

graphql_object!(User: RequestContext |&self| {
    description: "A user"

    field id(&executor) -> i32 {
        self.id
    }

    field display_name(&executor) -> &str {
        &self.display_name
    }

    field email(&executor) -> &str {
        &self.email
    }

    field profile_picture(&executor) -> Option<Medium> {
        let database: &diesel::PgConnection = &executor.context().conn;
        self.profile_picture(database)
    }

    field projects(&executor) -> Vec<Project> {
        let database_context = &executor.context().db_context_for_anon_user(ActionType::Read, ActionModifier::Own);
        let project_ids = Submission::belonging_to(self).select(submissions::project_id);
        projects::table
            .filter(projects::id.eq(any(project_ids)))
            .load::<Project>(database_context.conn).expect("Could not load projects")
    }
});
