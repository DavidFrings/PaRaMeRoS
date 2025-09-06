// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        img -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    events,
    users,
);
