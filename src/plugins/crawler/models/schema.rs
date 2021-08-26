table! {
    crawler_logs (id) {
        id -> Int8,
        site_id -> Int8,
        body -> Text,
        created_at -> Timestamp,
    }
}

table! {
    crawler_sites (id) {
        id -> Int8,
        name -> Varchar,
        url -> Varchar,
        cron -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(crawler_logs, crawler_sites,);
