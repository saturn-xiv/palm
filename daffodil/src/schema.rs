// @generated automatically by Diesel CLI.

diesel::table! {
    attachment_resources (id) {
        id -> Int4,
        attachment_id -> Int4,
        #[max_length = 127]
        resource_type -> Varchar,
        resource_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    attachments (id) {
        id -> Int4,
        user_id -> Int4,
        public -> Bool,
        #[max_length = 63]
        bucket -> Varchar,
        #[max_length = 63]
        object -> Varchar,
        #[max_length = 127]
        title -> Varchar,
        size -> Int4,
        #[max_length = 63]
        content_type -> Varchar,
        uploaded_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    email_users (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 63]
        real_name -> Varchar,
        #[max_length = 63]
        nickname -> Varchar,
        #[max_length = 127]
        email -> Varchar,
        password -> Bytea,
        salt -> Bytea,
        #[max_length = 255]
        avatar -> Varchar,
        confirmed_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    google_oauth2_users (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 127]
        subject -> Varchar,
        #[max_length = 127]
        email -> Nullable<Varchar>,
        email_verified -> Bool,
        #[max_length = 63]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        picture -> Nullable<Varchar>,
        #[max_length = 15]
        locale -> Nullable<Varchar>,
        token -> Bytea,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    leave_words (id) {
        id -> Int4,
        #[max_length = 15]
        lang -> Varchar,
        #[max_length = 45]
        ip -> Varchar,
        body -> Text,
        #[max_length = 15]
        editor -> Varchar,
        #[max_length = 15]
        status -> Varchar,
        published_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
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
    logs (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 31]
        plugin -> Varchar,
        #[max_length = 45]
        ip -> Varchar,
        #[max_length = 8]
        level -> Varchar,
        #[max_length = 127]
        resource_type -> Varchar,
        resource_id -> Nullable<Int4>,
        message -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 36]
        uid -> Varchar,
        #[max_length = 127]
        provider_type -> Varchar,
        provider_id -> Int4,
        #[max_length = 45]
        ip -> Varchar,
        expires_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    settings (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 255]
        key -> Varchar,
        value -> Bytea,
        salt -> Nullable<Bytea>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 36]
        uid -> Varchar,
        #[max_length = 15]
        lang -> Varchar,
        #[max_length = 31]
        timezone -> Varchar,
        sign_in_count -> Int4,
        current_sign_in_at -> Nullable<Timestamp>,
        #[max_length = 45]
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_at -> Nullable<Timestamp>,
        #[max_length = 45]
        last_sign_in_ip -> Nullable<Varchar>,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    wechat_mini_program_users (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 127]
        union_id -> Varchar,
        #[max_length = 63]
        app_id -> Varchar,
        #[max_length = 127]
        open_id -> Varchar,
        #[max_length = 63]
        nickname -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    wechat_oauth2_users (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 127]
        union_id -> Varchar,
        #[max_length = 63]
        app_id -> Varchar,
        #[max_length = 127]
        open_id -> Varchar,
        #[max_length = 63]
        nickname -> Varchar,
        sex -> Int2,
        #[max_length = 63]
        city -> Varchar,
        #[max_length = 63]
        province -> Varchar,
        #[max_length = 63]
        country -> Varchar,
        #[max_length = 255]
        head_img_url -> Nullable<Varchar>,
        privilege -> Bytea,
        #[max_length = 7]
        lang -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    attachment_resources,
    attachments,
    email_users,
    google_oauth2_users,
    leave_words,
    locales,
    logs,
    sessions,
    settings,
    users,
    wechat_mini_program_users,
    wechat_oauth2_users,
);
