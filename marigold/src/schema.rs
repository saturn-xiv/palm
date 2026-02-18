// @generated automatically by Diesel CLI.

diesel::table! {
    attachments (id) {
        id -> Int8,
        version -> Int8,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        user_id -> Int8,
        #[max_length = 127]
        title -> Varchar,
        #[max_length = 63]
        bucket -> Varchar,
        #[max_length = 63]
        object -> Varchar,
        #[max_length = 127]
        content_type -> Varchar,
        size -> Int8,
        public -> Bool,
        uploaded_at -> Nullable<Timestamptz>,
        expire_after_days -> Nullable<Int8>,
    }
}

diesel::table! {
    attachments_resources (id) {
        id -> Int8,
        attachment_id -> Int8,
        #[max_length = 127]
        resource_type -> Varchar,
        resource_id -> Nullable<Int8>,
        created_at -> Timestamptz,
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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    categories_resources (id) {
        id -> Int8,
        category_id -> Int8,
        #[max_length = 127]
        resource_type -> Varchar,
        resource_id -> Nullable<Int8>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    comments (id) {
        id -> Int8,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
        #[max_length = 127]
        resource_type -> Varchar,
        resource_id -> Int8,
        comment_id -> Nullable<Int8>,
        user_id -> Nullable<Int8>,
        body -> Text,
        editor -> Int4,
        #[max_length = 45]
        ip -> Varchar,
        #[max_length = 255]
        location -> Nullable<Varchar>,
    }
}

diesel::table! {
    email_users (id) {
        id -> Int8,
        version -> Int8,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        user_id -> Int8,
        #[max_length = 63]
        name -> Varchar,
        #[max_length = 31]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 127]
        avatar -> Nullable<Varchar>,
        confirmed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    google_oauth2_users (id) {
        id -> Int8,
        version -> Int8,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        user_id -> Int8,
        #[max_length = 127]
        code -> Varchar,
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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        level -> Int4,
        message -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    settings (id) {
        id -> Int8,
        version -> Int8,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        user_id -> Nullable<Int8>,
        #[max_length = 255]
        key -> Varchar,
        value -> Bytea,
        salt -> Nullable<Bytea>,
    }
}

diesel::table! {
    shorten_link (id) {
        id -> Int8,
        version -> Int8,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        #[max_length = 127]
        url -> Varchar,
        #[max_length = 63]
        title -> Varchar,
        #[max_length = 511]
        memo -> Varchar,
    }
}

diesel::table! {
    tags (id) {
        id -> Int8,
        version -> Int8,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        #[max_length = 63]
        name -> Varchar,
    }
}

diesel::table! {
    tags_resources (id) {
        id -> Int8,
        tag_id -> Int8,
        #[max_length = 127]
        resource_type -> Varchar,
        resource_id -> Nullable<Int8>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        version -> Int8,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
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
    }
}

diesel::table! {
    votes (id) {
        id -> Int8,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
        #[max_length = 127]
        resource_type -> Varchar,
        resource_id -> Int8,
        user_id -> Int8,
        value -> Int8,
        #[max_length = 255]
        location -> Nullable<Varchar>,
        #[max_length = 45]
        ip -> Varchar,
    }
}

diesel::table! {
    wechat_mini_program_users (id) {
        id -> Int8,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
        user_id -> Int8,
        #[max_length = 127]
        union_id -> Varchar,
        #[max_length = 127]
        app_id -> Varchar,
        #[max_length = 127]
        open_id -> Varchar,
        #[max_length = 63]
        nickname -> Nullable<Varchar>,
        #[max_length = 127]
        avatar_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    wechat_oauth2_users (id) {
        id -> Int8,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
        user_id -> Int8,
        #[max_length = 127]
        union_id -> Varchar,
        #[max_length = 127]
        app_id -> Varchar,
        #[max_length = 127]
        open_id -> Nullable<Varchar>,
        #[max_length = 63]
        nickname -> Varchar,
        sex -> Int8,
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
    }
}

diesel::joinable!(attachments -> users (user_id));
diesel::joinable!(attachments_resources -> attachments (attachment_id));
diesel::joinable!(categories_resources -> categories (category_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(email_users -> users (user_id));
diesel::joinable!(google_oauth2_users -> users (user_id));
diesel::joinable!(logs -> users (user_id));
diesel::joinable!(tags_resources -> tags (tag_id));
diesel::joinable!(votes -> users (user_id));
diesel::joinable!(wechat_mini_program_users -> users (user_id));
diesel::joinable!(wechat_oauth2_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    attachments,
    attachments_resources,
    casbin_rule,
    categories,
    categories_resources,
    comments,
    email_users,
    google_oauth2_users,
    locales,
    logs,
    settings,
    shorten_link,
    tags,
    tags_resources,
    users,
    votes,
    wechat_mini_program_users,
    wechat_oauth2_users,
);
