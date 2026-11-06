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
        media_creator -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        heading_en -> Nullable<Varchar>,
        content_en -> Nullable<Varchar>,
        season -> Nullable<Varchar>,
        content_html_backup -> Nullable<Varchar>,
    }
}

diesel::table! {
    tournament (id) {
        id -> Int4,
        name -> Varchar,
        place -> Nullable<Varchar>,
        starts_at -> Timestamptz,
        ends_at -> Nullable<Timestamptz>,
        updated_at -> Timestamptz,
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

diesel::allow_tables_to_appear_in_same_query!(posts, tournament, users,);
