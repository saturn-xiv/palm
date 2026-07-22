// @generated automatically by Diesel CLI.

diesel::table! {
    currencies (id) {
        id -> Int8,
        #[max_length = 127]
        name -> Varchar,
        #[max_length = 3]
        code -> Varchar,
        #[max_length = 127]
        country -> Varchar,
        number -> Int4,
        units -> Nullable<Int4>,
        fund -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    locales (id) {
        id -> Int8,
        #[max_length = 7]
        lang -> Varchar,
        #[max_length = 255]
        code -> Varchar,
        message -> Text,
        version -> Int8,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    logs (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 31]
        plugin -> Varchar,
        #[max_length = 7]
        level -> Varchar,
        #[max_length = 45]
        ip -> Varchar,
        message -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    schema_migrations (version) {
        version -> Varchar,
    }
}

diesel::table! {
    user_bans (id) {
        id -> Int8,
        user_id -> Int8,
        creator_id -> Int8,
        #[max_length = 45]
        ip -> Varchar,
        reason -> Text,
        expired_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_contacts (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 15]
        code -> Varchar,
        value -> Bytea,
        deleted_at -> Nullable<Timestamp>,
        version -> Int8,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 36]
        uid -> Varchar,
        #[max_length = 31]
        name -> Nullable<Varchar>,
        #[max_length = 127]
        avatar -> Nullable<Varchar>,
        #[max_length = 7]
        lang -> Varchar,
        #[max_length = 31]
        timezone -> Varchar,
        sign_in_count -> Int8,
        current_sign_in_at -> Nullable<Timestamp>,
        #[max_length = 45]
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_at -> Nullable<Timestamp>,
        #[max_length = 45]
        last_sign_in_ip -> Nullable<Varchar>,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int8,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::joinable!(logs -> users (user_id));
diesel::joinable!(user_contacts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    currencies,
    locales,
    logs,
    schema_migrations,
    user_bans,
    user_contacts,
    users,
);
