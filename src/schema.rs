table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    permissions (id) {
        id -> Int4,
        role_id -> Int4,
        resource_name -> Nullable<Varchar>,
        action -> Nullable<Action_type>,
        modifier -> Nullable<Action_modifier>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    projects (id) {
        id -> Int4,
        name -> Varchar,
        repository_url -> Varchar,
        color -> Varchar,
        description -> Nullable<Varchar>,
        slug -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    submissions (id) {
        id -> Int4,
        user_id -> Int4,
        project_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    user_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    users (id) {
        id -> Int4,
        display_name -> Varchar,
        email -> Varchar,
        password_digest -> Varchar,
    }
}

joinable!(permissions -> roles (role_id));
joinable!(submissions -> projects (project_id));
joinable!(submissions -> users (user_id));
joinable!(user_roles -> roles (role_id));
joinable!(user_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(permissions, projects, roles, submissions, user_roles, users,);
