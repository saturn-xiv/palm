// @generated automatically by Diesel CLI.

diesel::table! {
    cms_pages (id) {
        id -> Int4,
        #[max_length = 15]
        lang -> Varchar,
        parent_id -> Int4,
        #[max_length = 31]
        slug -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        #[max_length = 15]
        body_editor -> Varchar,
        #[max_length = 15]
        template -> Varchar,
        profile -> Bytea,
        #[max_length = 15]
        status -> Varchar,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}
