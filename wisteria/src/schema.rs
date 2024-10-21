// @generated automatically by Diesel CLI.

diesel::table! {
    questionnaire_fields (id) {
        id -> Int4,
        user_id -> Int4,
        form_id -> Int4,
        #[max_length = 36]
        uid -> Varchar,
        #[max_length = 255]
        label -> Varchar,
        summary -> Text,
        profile -> Bytea,
        sort_order -> Int4,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    questionnaire_forms (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 36]
        uid -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        #[max_length = 15]
        description_editor -> Varchar,
        profile -> Bytea,
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
    questionnaire_polls (id) {
        id -> Int4,
        form_id -> Int4,
        #[max_length = 36]
        batch_no -> Varchar,
        field_id -> Int4,
        value -> Bytea,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    questionnaire_fields,
    questionnaire_forms,
    questionnaire_polls,
);
