// @generated automatically by Diesel CLI.

diesel::table! {
    daffodil_bills (id) {
        id -> Int4,
        user_id -> Int4,
        ledger_id -> Int4,
        #[max_length = 511]
        summary -> Varchar,
        price -> Money,
        #[max_length = 3]
        currency -> Bpchar,
        #[max_length = 63]
        merchant -> Varchar,
        #[max_length = 31]
        category -> Varchar,
        paid_at -> Timestamp,
        #[max_length = 31]
        paid_by -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    daffodil_bills_history (id) {
        id -> Int4,
        ledger_id -> Int4,
        bill_id -> Int4,
        user_id -> Int4,
        #[max_length = 511]
        summary -> Varchar,
        price -> Money,
        #[max_length = 3]
        currency -> Bpchar,
        #[max_length = 63]
        merchant -> Varchar,
        #[max_length = 31]
        category -> Varchar,
        paid_at -> Timestamp,
        #[max_length = 31]
        paid_by -> Varchar,
        #[max_length = 255]
        reason -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    daffodil_ledgers (id) {
        id -> Int4,
        owner_id -> Int4,
        #[max_length = 36]
        uid -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 511]
        summary -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    daffodil_bills,
    daffodil_bills_history,
    daffodil_ledgers,
);
