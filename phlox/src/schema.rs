// @generated automatically by Diesel CLI.

diesel::table! {
    casbin_rule (id) {
        id -> Int4,
        #[max_length = 127]
        ptype -> Varchar,
        #[max_length = 127]
        v0 -> Varchar,
        #[max_length = 127]
        v1 -> Varchar,
        #[max_length = 127]
        v2 -> Varchar,
        #[max_length = 127]
        v3 -> Varchar,
        #[max_length = 127]
        v4 -> Varchar,
        #[max_length = 127]
        v5 -> Varchar,
    }
}

diesel::table! {
    currencies (id) {
        id -> Int4,
        #[max_length = 3]
        code -> Bpchar,
        #[max_length = 3]
        number -> Bpchar,
        #[max_length = 127]
        name -> Varchar,
        #[max_length = 127]
        country -> Varchar,
        units -> Int4,
        is_fund -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    locales (id) {
        id -> Int4,
        #[max_length = 15]
        lang -> Varchar,
        #[max_length = 255]
        code -> Varchar,
        message -> Text,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    schema_migrations (version) {
        #[max_length = 128]
        version -> Varchar,
    }
}

diesel::table! {
    settings (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 255]
        key -> Varchar,
        value -> Bytea,
        nonce -> Nullable<Bytea>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    casbin_rule,
    currencies,
    locales,
    schema_migrations,
    settings,
);
