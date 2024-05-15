// @generated automatically by Diesel CLI.

diesel::table! {
    daffodil_accounts (id) {
        id -> Int8,
        user_id -> Int8,
        book_id -> Int8,
        #[max_length = 63]
        name -> Varchar,
        #[max_length = 3]
        currency -> Bpchar,
        #[sql_name = "type"]
        type_ -> Int4,
        #[max_length = 511]
        description -> Varchar,
        #[max_length = 255]
        cover -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    daffodil_books (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 63]
        name -> Varchar,
        #[max_length = 511]
        description -> Varchar,
        #[max_length = 255]
        cover -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    daffodil_merchants (id) {
        id -> Int8,
        user_id -> Int8,
        book_id -> Int8,
        #[max_length = 63]
        name -> Varchar,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 255]
        contact -> Nullable<Varchar>,
        #[max_length = 511]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        cover -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    daffodil_transaction_attachments (id) {
        id -> Int8,
        user_id -> Int8,
        transaction_id -> Int8,
        #[max_length = 63]
        bucket -> Varchar,
        #[max_length = 63]
        object -> Varchar,
        #[max_length = 63]
        title -> Varchar,
        #[max_length = 63]
        content_type -> Varchar,
        published_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    daffodil_transaction_trash (id) {
        id -> Int8,
        transaction_id -> Int8,
        user_id -> Int8,
        book_id -> Int8,
        source_account_id -> Int8,
        destination_account_id -> Int8,
        merchant_id -> Int8,
        #[sql_name = "type"]
        type_ -> Int8,
        amount -> Int8,
        #[max_length = 3]
        currency -> Varchar,
        #[max_length = 511]
        summary -> Varchar,
        #[max_length = 255]
        deleted_reason -> Varchar,
        deleted_by -> Int8,
        deleted_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    daffodil_transactions (id) {
        id -> Int8,
        user_id -> Int8,
        book_id -> Int8,
        source_account_id -> Int8,
        destination_account_id -> Int8,
        merchant_id -> Int8,
        #[sql_name = "type"]
        type_ -> Int8,
        amount -> Int8,
        #[max_length = 3]
        currency -> Varchar,
        #[max_length = 511]
        summary -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    daffodil_accounts,
    daffodil_books,
    daffodil_merchants,
    daffodil_transaction_attachments,
    daffodil_transaction_trash,
    daffodil_transactions,
);
