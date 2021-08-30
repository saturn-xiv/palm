table! {
    groups (id) {
        id -> Int8,
        code -> Varchar,
        name -> Varchar,
        parent_id -> Nullable<Int8>,
        level -> Int8,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    groups_users (id) {
        id -> Int8,
        group_id -> Int8,
        user_id -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> Int8,
        user_id -> Int8,
        ip -> Varchar,
        message -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    operations (id) {
        id -> Int8,
        code -> Varchar,
        name -> Varchar,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    policies (id) {
        id -> Int8,
        role_id -> Int8,
        resource_id -> Int8,
        operation_id -> Int8,
        not_before -> Date,
        expire_at -> Date,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    resources (id) {
        id -> Int8,
        #[sql_name = "type"]
        type_ -> Varchar,
        code -> Varchar,
        name -> Varchar,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    roles (id) {
        id -> Int8,
        code -> Varchar,
        name -> Varchar,
        parent_id -> Nullable<Int8>,
        level -> Int8,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    roles_groups (id) {
        id -> Int8,
        role_id -> Int8,
        group_id -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    roles_relations (id) {
        id -> Int8,
        a -> Int8,
        constraint -> Varchar,
        b -> Int8,
        name -> Varchar,
        version -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    roles_users (id) {
        id -> Int8,
        role_id -> Int8,
        user_id -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int8,
        real_name -> Varchar,
        nick_name -> Varchar,
        email -> Varchar,
        password -> Nullable<Bytea>,
        salt -> Nullable<Bytea>,
        uid -> Varchar,
        provider_type -> Varchar,
        provider_id -> Varchar,
        access_token -> Nullable<Varchar>,
        logo -> Varchar,
        lang -> Varchar,
        profile -> Bytea,
        sign_in_count -> Int8,
        current_sign_in_at -> Nullable<Timestamp>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_at -> Nullable<Timestamp>,
        last_sign_in_ip -> Nullable<Varchar>,
        confirmed_at -> Nullable<Timestamp>,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    groups,
    groups_users,
    logs,
    operations,
    policies,
    resources,
    roles,
    roles_groups,
    roles_relations,
    roles_users,
    users,
);
