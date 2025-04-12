// @generated automatically by Diesel CLI.

diesel::table! {
    libraries (id) {
        id -> BigInt,
        created_at -> Text,
        updated_at -> Text,
        name -> Text,
        path -> Text,
        media_type -> Text,
    }
}

diesel::table! {
    media (id) {
        id -> BigInt,
        created_at -> Text,
        updated_at -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        nfo_id -> Nullable<BigInt>,
        path -> Text,
        video_file -> Nullable<Text>,
        title -> Text,
        season -> Nullable<Integer>,
        episode -> Nullable<Integer>,
        attributes -> Text,
        library_id -> BigInt,
        parent_id -> Nullable<BigInt>,
    }
}

diesel::joinable!(media -> libraries (library_id));

diesel::allow_tables_to_appear_in_same_query!(
    libraries,
    media,
);
