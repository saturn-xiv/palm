// @generated automatically by Diesel CLI.

diesel::table! {
    cms_pages (id) {
        id -> Int8,
        author_id -> Int8,
        #[max_length = 63]
        slug -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 8]
        lang -> Varchar,
        #[max_length = 511]
        summary -> Varchar,
        body -> Text,
        body_editor -> Int4,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}
