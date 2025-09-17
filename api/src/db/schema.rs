// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        uuid -> Uuid,
        name -> Varchar,
        heading -> Varchar,
        content -> Varchar,
        media_type -> Nullable<Varchar>,
        media_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        admin -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
