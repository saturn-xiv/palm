// @generated automatically by Diesel CLI.

diesel::table! {
    forum_posts (id) {
        id -> Int4,
        author_id -> Int4,
        topic_id -> Int4,
        post_id -> Nullable<Int4>,
        #[max_length = 8]
        lang -> Varchar,
        body -> Text,
        #[max_length = 15]
        body_editor -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    forum_topics (id) {
        id -> Int4,
        author_id -> Int4,
        #[max_length = 63]
        slug -> Varchar,
        #[max_length = 255]
        subject -> Varchar,
        #[max_length = 8]
        lang -> Varchar,
        body -> Text,
        #[max_length = 15]
        body_editor -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(forum_posts, forum_topics,);
