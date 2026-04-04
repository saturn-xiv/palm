use std::sync::Arc;

use phlox::{
    Result,
    casbin::{Enforcer, RbacApi, postgresql_rabbitmq_enforcer},
    orm::postgresql::Node as PostgreSql,
    queue::rabbitmq::Node as RabbitMq,
};
use tokio::{
    sync::Mutex,
    time::{Duration, sleep},
};

const USER: &str = "u.bbb";
const ROLE: &str = "r.admin";

#[tokio::test]
async fn watcher() {
    let enforcer_a = open_enforcer().await.unwrap();
    let enforcer_b = open_enforcer().await.unwrap();

    {
        let mut enf = enforcer_a.lock().await;
        enf.delete_role(ROLE).await.unwrap();
        assert!(!enf.has_role_for_user(USER, ROLE, None));
    }
    sleep(Duration::from_secs(1)).await;
    {
        let enf = enforcer_b.lock().await;
        assert!(!enf.has_role_for_user(USER, ROLE, None));
    }
    sleep(Duration::from_secs(1)).await;
    {
        let mut enf = enforcer_a.lock().await;
        enf.add_role_for_user(USER, ROLE, None).await.unwrap();
    }
    sleep(Duration::from_secs(1)).await;
    {
        let enf = enforcer_b.lock().await;
        assert!(enf.has_role_for_user(USER, ROLE, None));
    }
}

async fn open_enforcer() -> Result<Arc<Mutex<Enforcer>>> {
    let db = PostgreSql {
        host: "127.0.0.1".to_string(),
        port: 5432,
        user: "www".to_string(),
        password: Some("change-me".to_string()),
        db_name: "phlox_testing".to_string(),
        pool_size: 16,
    };
    let queue = RabbitMq {
        host: "127.0.0.1".to_string(),
        port: 5672,
        user: "www".to_string(),
        password: "change-me".to_string(),
        virtual_host: "testing".to_string(),
    };
    postgresql_rabbitmq_enforcer(db.open()?, Arc::new(queue.open().await?)).await
}
