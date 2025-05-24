// @generated automatically by Diesel CLI.

diesel::table! {
    logs (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        message -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    schema_migrations (version) {
        version -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        password -> Text,
        profile -> Binary,
        deleted_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(logs, schema_migrations, users,);
