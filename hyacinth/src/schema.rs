// @generated automatically by Diesel CLI.

diesel::table! {
    attachments (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 63]
        bucket -> Varchar,
        #[max_length = 63]
        object -> Varchar,
        #[max_length = 127]
        title -> Varchar,
        size -> Int8,
        #[max_length = 63]
        content_type -> Varchar,
        public -> Bool,
        uploaded_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Int8,
        #[max_length = 31]
        code -> Varchar,
        left -> Int4,
        right -> Int4,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    currencies (id) {
        id -> Int8,
        #[max_length = 127]
        name -> Varchar,
        #[max_length = 3]
        code -> Varchar,
        #[max_length = 127]
        country -> Varchar,
        number -> Int4,
        units -> Nullable<Int4>,
        fund -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    email_users (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 63]
        email -> Varchar,
        #[max_length = 127]
        password -> Varchar,
        #[max_length = 127]
        avatar -> Varchar,
        confirmed_at -> Nullable<Timestamp>,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    google_users (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 127]
        email -> Nullable<Varchar>,
        email_verified -> Bool,
        #[max_length = 63]
        name -> Nullable<Varchar>,
        #[max_length = 127]
        picture -> Nullable<Varchar>,
        #[max_length = 127]
        sub -> Varchar,
        code -> Bytea,
        #[max_length = 127]
        token -> Varchar,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    locales (id) {
        id -> Int8,
        #[max_length = 7]
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
        id -> Int8,
        user_id -> Int8,
        #[max_length = 31]
        plugin -> Varchar,
        #[max_length = 7]
        level -> Varchar,
        #[max_length = 45]
        ip -> Varchar,
        message -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    schema_migrations (version) {
        version -> Varchar,
    }
}

diesel::table! {
    settings (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        #[max_length = 255]
        key -> Varchar,
        value -> Bytea,
        associated_data -> Nullable<Bytea>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    shorten_links (id) {
        id -> Int8,
        #[max_length = 127]
        title -> Varchar,
        #[max_length = 127]
        url -> Varchar,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Int8,
        #[max_length = 31]
        code -> Varchar,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_bans (id) {
        id -> Int8,
        user_id -> Int8,
        creator_id -> Int8,
        #[max_length = 45]
        ip -> Varchar,
        reason -> Text,
        expired_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_contacts (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 15]
        code -> Varchar,
        value -> Bytea,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 36]
        uid -> Varchar,
        #[max_length = 31]
        name -> Varchar,
        #[max_length = 127]
        avatar -> Nullable<Varchar>,
        #[max_length = 7]
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
    votes (id) {
        id -> Int8,
        user_id -> Int8,
        stars -> Int4,
        comment -> Text,
        #[max_length = 7]
        editor -> Varchar,
        published_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    wechat_mini_program_users (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 127]
        union_id -> Varchar,
        #[max_length = 63]
        app_id -> Varchar,
        #[max_length = 63]
        open_id -> Varchar,
        #[max_length = 63]
        nickname -> Nullable<Varchar>,
        #[max_length = 127]
        head_img_url -> Nullable<Varchar>,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    wechat_oauth2_users (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 127]
        union_id -> Varchar,
        #[max_length = 63]
        app_id -> Varchar,
        #[max_length = 63]
        open_id -> Varchar,
        #[max_length = 63]
        nickname -> Varchar,
        sex -> Int4,
        #[max_length = 63]
        city -> Varchar,
        #[max_length = 63]
        province -> Varchar,
        #[max_length = 63]
        country -> Varchar,
        #[max_length = 127]
        head_img_url -> Nullable<Varchar>,
        privilege -> Bytea,
        #[max_length = 7]
        lang -> Varchar,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::joinable!(attachments -> users (user_id));
diesel::joinable!(email_users -> users (user_id));
diesel::joinable!(google_users -> users (user_id));
diesel::joinable!(logs -> users (user_id));
diesel::joinable!(settings -> users (user_id));
diesel::joinable!(user_contacts -> users (user_id));
diesel::joinable!(votes -> users (user_id));
diesel::joinable!(wechat_mini_program_users -> users (user_id));
diesel::joinable!(wechat_oauth2_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    attachments,
    categories,
    currencies,
    email_users,
    google_users,
    locales,
    logs,
    schema_migrations,
    settings,
    shorten_links,
    tags,
    user_bans,
    user_contacts,
    users,
    votes,
    wechat_mini_program_users,
    wechat_oauth2_users,
);
