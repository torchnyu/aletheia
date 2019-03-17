table! {
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
    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    submissions (id) {
        id -> Int4,
        user_id -> Int4,
        project_id -> Int4,
    }
}

table! {
    user_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        display_name -> Varchar,
        email -> Varchar,
        password_digest -> Varchar,
    }
}

joinable!(submissions -> projects (project_id));
joinable!(submissions -> users (user_id));
joinable!(user_roles -> roles (role_id));
joinable!(user_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    projects,
    roles,
    submissions,
    user_roles,
    users,
);
