// @generated automatically by Diesel CLI.

diesel::table! {
    bookkeeper_accounts (id) {
        id -> Int4,
        ledger_id -> Int4,
        parent_id -> Nullable<Int4>,
        #[max_length = 63]
        label -> Varchar,
        #[max_length = 1023]
        memo -> Varchar,
        currency_id -> Int4,
        #[sql_name = "type"]
        #[max_length = 15]
        type_ -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    bookkeeper_categories (id) {
        id -> Int4,
        ledger_id -> Int4,
        parent_id -> Nullable<Int4>,
        #[max_length = 63]
        label -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    bookkeeper_entries (id) {
        id -> Int4,
        ledger_id -> Int4,
        #[max_length = 18]
        sn -> Bpchar,
        transaction_id -> Int4,
        from_account_id -> Int4,
        to_account_id -> Int4,
        category_id -> Int4,
        merchant_id -> Int4,
        currency_id -> Int4,
        amount -> Int4,
        #[max_length = 1023]
        memo -> Varchar,
        traded_at -> Timestamp,
        #[max_length = 31]
        timezone -> Varchar,
        #[max_length = 31]
        status -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    bookkeeper_ledgers (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 36]
        uid -> Varchar,
        #[max_length = 63]
        label -> Varchar,
        #[max_length = 1023]
        memo -> Varchar,
        profile -> Bytea,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    bookkeeper_logs (id) {
        id -> Int4,
        ledger_id -> Int4,
        user_id -> Int4,
        #[max_length = 511]
        username -> Varchar,
        #[max_length = 31]
        action -> Varchar,
        memo -> Text,
        #[max_length = 255]
        reason -> Nullable<Varchar>,
        #[max_length = 45]
        ip -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    bookkeeper_merchants (id) {
        id -> Int4,
        ledger_id -> Int4,
        #[max_length = 63]
        label -> Varchar,
        #[max_length = 1023]
        memo -> Varchar,
        address -> Nullable<Int4>,
        contact -> Nullable<Int4>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    bookkeeper_statements (id) {
        id -> Int4,
        ledger_id -> Int4,
        account_id -> Int4,
        transaction_id -> Int4,
        entry_id -> Int4,
        currency_id -> Int4,
        amount -> Int4,
        #[sql_name = "type"]
        #[max_length = 7]
        type_ -> Varchar,
        opening_balance -> Int4,
        closing_balance -> Int4,
        traded_at -> Timestamp,
        #[max_length = 31]
        timezone -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    bookkeeper_transactions (id) {
        id -> Int4,
        #[max_length = 36]
        uid -> Varchar,
        ledger_id -> Int4,
        #[max_length = 1023]
        memo -> Varchar,
        traded_at -> Timestamp,
        #[max_length = 31]
        timezone -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bookkeeper_accounts,
    bookkeeper_categories,
    bookkeeper_entries,
    bookkeeper_ledgers,
    bookkeeper_logs,
    bookkeeper_merchants,
    bookkeeper_statements,
    bookkeeper_transactions,
);
