// @generated automatically by Diesel CLI.

diesel::table! {
    attachments (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        user_id -> Int8,
        #[max_length = 127]
        title -> Varchar,
        #[max_length = 63]
        bucket -> Varchar,
        #[max_length = 63]
        object -> Varchar,
        #[max_length = 63]
        content_type -> Varchar,
        size -> Int8,
        public -> Bool,
        version -> Int8,
    }
}

diesel::table! {
    attachments_resources (id) {
        id -> Int8,
        attachment_id -> Int8,
        #[max_length = 127]
        resource_type -> Varchar,
        resource_id -> Nullable<Int8>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    casbin_rule (id) {
        id -> Int8,
        #[max_length = 100]
        ptype -> Nullable<Varchar>,
        #[max_length = 100]
        v0 -> Nullable<Varchar>,
        #[max_length = 100]
        v1 -> Nullable<Varchar>,
        #[max_length = 100]
        v2 -> Nullable<Varchar>,
        #[max_length = 100]
        v3 -> Nullable<Varchar>,
        #[max_length = 100]
        v4 -> Nullable<Varchar>,
        #[max_length = 100]
        v5 -> Nullable<Varchar>,
    }
}

diesel::table! {
    categories (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        left -> Int8,
        right -> Int8,
        version -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    categories_resources (id) {
        id -> Int8,
        category_id -> Int8,
        #[max_length = 127]
        resource_type -> Varchar,
        resource_id -> Nullable<Int8>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    email_users (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        user_id -> Int8,
        #[max_length = 36]
        sn -> Varchar,
        #[max_length = 63]
        name -> Varchar,
        #[max_length = 31]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 127]
        avatar -> Nullable<Varchar>,
        confirmed_at -> Nullable<Timestamptz>,
        version -> Int8,
    }
}

diesel::table! {
    google_oauth2_users (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        user_id -> Int8,
        #[max_length = 127]
        code -> Varchar,
        #[max_length = 36]
        sn -> Varchar,
        #[max_length = 63]
        name -> Varchar,
        #[max_length = 63]
        email -> Varchar,
        email_verified -> Nullable<Bool>,
        #[max_length = 127]
        picture -> Varchar,
        #[max_length = 15]
        gender -> Varchar,
        #[max_length = 127]
        link -> Varchar,
        #[max_length = 15]
        locale -> Varchar,
        version -> Int8,
    }
}

diesel::table! {
    locales (id) {
        id -> Int8,
        #[max_length = 15]
        lang -> Varchar,
        #[max_length = 255]
        code -> Varchar,
        message -> Text,
        version -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    logs (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 15]
        plugin -> Varchar,
        #[max_length = 45]
        ip -> Varchar,
        #[max_length = 7]
        level -> Varchar,
        message -> Text,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    settings (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        key -> Varchar,
        value -> Bytea,
        salt -> Nullable<Bytea>,
        version -> Int8,
    }
}

diesel::table! {
    tags (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        name -> Varchar,
        version -> Int8,
    }
}

diesel::table! {
    tags_resources (id) {
        id -> Int8,
        tag_id -> Int8,
        #[max_length = 127]
        resource_type -> Varchar,
        resource_id -> Nullable<Int8>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        #[max_length = 36]
        sn -> Varchar,
        #[max_length = 15]
        lang -> Varchar,
        #[max_length = 31]
        timezone -> Varchar,
        signed_in_total -> Int8,
        current_signed_in_at -> Nullable<Timestamptz>,
        #[max_length = 45]
        current_signed_in_ip -> Nullable<Varchar>,
        last_signed_in_at -> Nullable<Timestamptz>,
        #[max_length = 45]
        last_signed_in_ip -> Nullable<Varchar>,
        locked_at -> Nullable<Timestamptz>,
        version -> Int8,
    }
}

diesel::joinable!(attachments -> users (user_id));
diesel::joinable!(attachments_resources -> attachments (attachment_id));
diesel::joinable!(categories_resources -> categories (category_id));
diesel::joinable!(email_users -> users (user_id));
diesel::joinable!(google_oauth2_users -> users (user_id));
diesel::joinable!(logs -> users (user_id));
diesel::joinable!(tags_resources -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    attachments,
    attachments_resources,
    casbin_rule,
    categories,
    categories_resources,
    email_users,
    google_oauth2_users,
    locales,
    logs,
    settings,
    tags,
    tags_resources,
    users,
);
