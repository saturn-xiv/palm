// @generated automatically by Diesel CLI.

diesel::table! {
    schema_migrations (version) {
        #[max_length = 128]
        version -> Varchar,
    }
}
