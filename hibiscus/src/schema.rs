// @generated automatically by Diesel CLI.

diesel::table! {
    bbs_forums (id) {
        id -> Int4,
        #[max_length = 15]
        lang -> Varchar,
        #[max_length = 31]
        slug -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 511]
        description -> Varchar,
        #[max_length = 15]
        status -> Varchar,
        profile -> Bytea,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    bbs_posts (id) {
        id -> Int4,
        forum_id -> Int4,
        topic_id -> Int4,
        parent_id -> Nullable<Int4>,
        body -> Text,
        #[max_length = 15]
        body_editor -> Varchar,
        author_id -> Int4,
        #[max_length = 15]
        status -> Varchar,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    bbs_topics (id) {
        id -> Int4,
        forum_id -> Int4,
        #[max_length = 127]
        slug -> Varchar,
        #[max_length = 255]
        subject -> Varchar,
        body -> Text,
        #[max_length = 15]
        body_editor -> Varchar,
        author_id -> Int4,
        #[max_length = 15]
        status -> Varchar,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(bbs_forums, bbs_posts, bbs_topics,);
