table! {
    contributors (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
    }
}

table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        repository_url -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    contributors,
    projects,
);
