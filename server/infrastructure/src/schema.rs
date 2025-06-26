// @generated automatically by Diesel CLI.

diesel::table! {
    libraries (id) {
        id -> Int8,
        created_at -> Timestamp,
        created_by -> Text,
        updated_at -> Timestamp,
        updated_by -> Text,
        name -> Text,
        path -> Text,
        media_type -> Text,
    }
}

diesel::table! {
    media (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        library_id -> Int8,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Varchar,
        path -> Nullable<Text>,
        title -> Text,
        season -> Nullable<Int4>,
        episode -> Nullable<Int4>,
        files -> Jsonb,
        attributes -> Jsonb,
        parent_id -> Nullable<Int8>,
    }
}

diesel::joinable!(media -> libraries (library_id));

diesel::allow_tables_to_appear_in_same_query!(
    libraries,
    media,
);
