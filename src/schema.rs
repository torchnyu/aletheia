table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        repository_url -> Varchar,
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

allow_tables_to_appear_in_same_query!(
    projects,
    users,
);
