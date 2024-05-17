// @generated automatically by Diesel CLI.

diesel::table! {
    attachment_resources (id) {
        id -> Int8,
        attachment_id -> Int8,
        #[max_length = 255]
        resource_type -> Varchar,
        resource_id -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    attachments (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 63]
        bucket -> Varchar,
        #[max_length = 63]
        name -> Varchar,
        #[max_length = 127]
        title -> Varchar,
        size -> Int8,
        #[max_length = 63]
        content_type -> Varchar,
        published_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Int8,
        #[max_length = 63]
        code -> Varchar,
        left -> Int4,
        right -> Int4,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    category_resources (id) {
        id -> Int8,
        category_id -> Int8,
        #[max_length = 255]
        resource_type -> Varchar,
        resource_id -> Int8,
        sort_order -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 63]
        username -> Varchar,
        #[max_length = 45]
        ip -> Varchar,
        comment_id -> Nullable<Int8>,
        content -> Text,
        content_editor -> Int4,
        #[max_length = 255]
        resource_type -> Varchar,
        resource_id -> Int8,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    crawler_logs (id) {
        id -> Int8,
        #[max_length = 255]
        url -> Varchar,
        body -> Bytea,
        created_at -> Timestamp,
    }
}

diesel::table! {
    email_users (id) {
        id -> Int8,
        user_id -> Int8,
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
    favorites (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 127]
        subject -> Varchar,
        #[max_length = 255]
        url -> Varchar,
        #[max_length = 255]
        resource_type -> Varchar,
        resource_id -> Int8,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    feedbacks (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 63]
        username -> Varchar,
        #[max_length = 15]
        mobile -> Varchar,
        #[max_length = 127]
        email -> Varchar,
        content -> Text,
        #[max_length = 15]
        content_editor -> Varchar,
        #[max_length = 255]
        resource_type -> Varchar,
        resource_id -> Int8,
        status -> Int4,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    footprints (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 255]
        resource_type -> Varchar,
        resource_id -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    google_users (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        email_verified -> Bool,
        #[max_length = 63]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        picture -> Nullable<Varchar>,
        #[max_length = 127]
        sub -> Varchar,
        code -> Bytea,
        #[max_length = 127]
        token -> Varchar,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    issues (id) {
        id -> Int8,
        #[max_length = 15]
        lang -> Varchar,
        #[max_length = 255]
        question -> Varchar,
        answer -> Text,
        answer_editor -> Int4,
        #[max_length = 255]
        resource_type -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    leave_words (id) {
        id -> Int8,
        #[max_length = 15]
        lang -> Varchar,
        #[max_length = 45]
        ip -> Varchar,
        body -> Text,
        body_editor -> Int4,
        status -> Int4,
        published_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    locales (id) {
        id -> Int8,
        #[max_length = 8]
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
        #[max_length = 15]
        plugin -> Varchar,
        level -> Int4,
        #[max_length = 45]
        ip -> Varchar,
        #[max_length = 255]
        resource_type -> Varchar,
        resource_id -> Nullable<Int8>,
        message -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    menus (id) {
        id -> Int8,
        #[max_length = 63]
        code -> Varchar,
        #[max_length = 31]
        location -> Varchar,
        left -> Int4,
        right -> Int4,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    notifications (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 127]
        subject -> Varchar,
        #[max_length = 511]
        body -> Varchar,
        #[max_length = 255]
        url -> Varchar,
        status -> Int4,
        read_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    search_histories (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 45]
        ip -> Varchar,
        #[max_length = 63]
        keyword -> Varchar,
        #[max_length = 255]
        from -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    settings (id) {
        id -> Int8,
        #[max_length = 255]
        key -> Varchar,
        user_id -> Nullable<Int8>,
        salt -> Nullable<Bytea>,
        value -> Bytea,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    shorter_links (id) {
        id -> Int8,
        #[max_length = 255]
        url -> Varchar,
        #[max_length = 511]
        summary -> Varchar,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tag_resources (id) {
        id -> Int8,
        tag_id -> Int8,
        #[max_length = 255]
        resource_type -> Varchar,
        resource_id -> Int8,
        sort_order -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Int8,
        #[max_length = 63]
        code -> Varchar,
        sort_order -> Int4,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_bans (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 45]
        ip -> Varchar,
        #[max_length = 255]
        reason -> Varchar,
        expired_at -> Timestamp,
        creator_id -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_contacts (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 255]
        key -> Varchar,
        value -> Bytea,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_sessions (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 36]
        uid -> Varchar,
        provider_type -> Int4,
        provider_id -> Int8,
        #[max_length = 45]
        ip -> Varchar,
        expired_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 36]
        uid -> Varchar,
        #[max_length = 31]
        name -> Nullable<Varchar>,
        #[max_length = 127]
        avatar -> Nullable<Varchar>,
        #[max_length = 8]
        lang -> Varchar,
        #[max_length = 32]
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
    vote_items (id) {
        id -> Int8,
        star_ -> Int4,
        #[max_length = 255]
        resource_type -> Varchar,
        resource_id -> Int8,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    vote_logs (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 45]
        ip -> Varchar,
        star_ -> Int4,
        comment -> Text,
        comment_editor -> Int4,
        #[max_length = 255]
        resource_type -> Varchar,
        resource_id -> Int8,
        status -> Int4,
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
        #[max_length = 127]
        open_id -> Varchar,
        #[max_length = 63]
        nickname -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
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
        #[max_length = 127]
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
        #[max_length = 255]
        head_img_url -> Nullable<Varchar>,
        privilege -> Bytea,
        #[max_length = 8]
        lang -> Varchar,
        version -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    attachment_resources,
    attachments,
    categories,
    category_resources,
    comments,
    crawler_logs,
    email_users,
    favorites,
    feedbacks,
    footprints,
    google_users,
    issues,
    leave_words,
    locales,
    logs,
    menus,
    notifications,
    search_histories,
    settings,
    shorter_links,
    tag_resources,
    tags,
    user_bans,
    user_contacts,
    user_sessions,
    users,
    vote_items,
    vote_logs,
    wechat_mini_program_users,
    wechat_oauth2_users,
);
