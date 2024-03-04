// @generated automatically by Diesel CLI.

diesel::table! {
    cms_pages (id) {
        id -> Int4,
        author_id -> Int4,
        #[max_length = 63]
        slug -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 8]
        lang -> Varchar,
        #[max_length = 511]
        summary -> Varchar,
        body -> Text,
        #[max_length = 15]
        body_editor -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}
