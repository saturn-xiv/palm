// @generated automatically by Diesel CLI.

diesel::table! {
    daffodil_bills (id) {
        id -> Int4,
        author_id -> Int4,
        ledger_id -> Int4,
        #[max_length = 511]
        summary -> Varchar,
        total -> Money,
        #[max_length = 3]
        currency -> Bpchar,
        #[max_length = 64]
        merchant -> Varchar,
        #[max_length = 32]
        category -> Varchar,
        paid_at -> Timestamp,
        #[max_length = 32]
        paid_by -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    daffodil_ledgers (id) {
        id -> Int4,
        owner_id -> Int4,
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
    daffodil_ledgers,
);
