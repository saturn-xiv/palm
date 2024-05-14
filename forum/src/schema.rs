// @generated automatically by Diesel CLI.

diesel::table! {
    forum_posts (id) {
        id -> Int8,
        author_id -> Int8,
        topic_id -> Int8,
        post_id -> Nullable<Int8>,
        #[max_length = 8]
        lang -> Varchar,
        body -> Text,
        body_editor -> Int4,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    forum_topics (id) {
        id -> Int8,
        author_id -> Int8,
        #[max_length = 63]
        slug -> Varchar,
        #[max_length = 255]
        subject -> Varchar,
        #[max_length = 8]
        lang -> Varchar,
        body -> Text,
        body_editor -> Int4,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    forum_posts,
    forum_topics,
);
