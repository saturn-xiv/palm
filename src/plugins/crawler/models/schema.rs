table! {
    crawler_logs (id) {
        id -> Int4,
        url -> Varchar,
        body -> Text,
        created_at -> Timestamp,
    }
}

table! {
    crawler_sites (id) {
        id -> Int4,
        name -> Varchar,
        url -> Varchar,
        ttl -> Int4,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(crawler_logs, crawler_sites,);
