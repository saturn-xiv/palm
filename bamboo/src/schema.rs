// @generated automatically by Diesel CLI.

diesel::table! {
    hosts (id) {
        id -> Int4,
        member_id -> Nullable<Int4>,
        #[max_length = 63]
        name -> Nullable<Varchar>,
        #[max_length = 17]
        mac -> Bpchar,
        #[max_length = 39]
        ip -> Varchar,
        fixed -> Bool,
        memo -> Text,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    hosts_rules (id) {
        id -> Int4,
        host_id -> Int4,
        rule_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    logs (id) {
        id -> Int4,
        user_id -> Int4,
        message -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    members (id) {
        id -> Int4,
        #[max_length = 31]
        sn -> Varchar,
        #[max_length = 31]
        name -> Varchar,
        #[max_length = 255]
        wifi_password -> Varchar,
        profile -> Bytea,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    rules (id) {
        id -> Int4,
        #[max_length = 31]
        name -> Varchar,
        content -> Bytea,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    settings (id) {
        id -> Int4,
        #[max_length = 255]
        key -> Varchar,
        salt -> Nullable<Bytea>,
        value -> Bytea,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 31]
        name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    hosts,
    hosts_rules,
    logs,
    members,
    rules,
    settings,
    users,
);
