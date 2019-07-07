table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    events (id) {
        id -> Int4,
        name -> Varchar,
        start_time -> Timestamp,
        end_time -> Timestamp,
        description -> Nullable<Varchar>,
        slug -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    media (id) {
        id -> Int4,
        folder_name -> Varchar,
        project_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    permissions (id) {
        id -> Int4,
        role_id -> Int4,
        action -> Array<Action_type>,
        modifier -> Array<Action_modifier>,
        resource_name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    projects (id) {
        id -> Int4,
        name -> Varchar,
        repository_url -> Varchar,
        description -> Nullable<Varchar>,
        slug -> Varchar,
        event_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    submissions (id) {
        id -> Int4,
        user_id -> Int4,
        project_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    user_events (id) {
        id -> Int4,
        user_id -> Int4,
        event_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    user_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    users (id) {
        id -> Int4,
        display_name -> Varchar,
        email -> Varchar,
        password_digest -> Varchar,
    }
}

joinable!(media -> projects (project_id));
joinable!(media -> users (user_id));
joinable!(permissions -> roles (role_id));
joinable!(submissions -> projects (project_id));
joinable!(submissions -> users (user_id));
joinable!(user_events -> events (event_id));
joinable!(user_events -> users (user_id));
joinable!(user_roles -> roles (role_id));
joinable!(user_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    events,
    media,
    permissions,
    projects,
    roles,
    submissions,
    user_events,
    user_roles,
    users,
);
