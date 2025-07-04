use std::sync::LazyLock;
use deadpool_redis::{Config, Pool, Runtime};

use crate::util::app_config::APP_CONFIG;

pub static REDIS_INSTANCE: LazyLock<RedisInstance> = LazyLock::new(|| RedisInstance::default());

type DeadpoolInstance = Pool;

pub struct RedisInstance {
    pub pool: DeadpoolInstance
}

impl Default for RedisInstance {
fn default() -> Self {
        let redis_cfg = APP_CONFIG.redis.clone();
        let uri = build_redis_uri(&redis_cfg.url, &redis_cfg.database, &redis_cfg.timeout_seconds, &redis_cfg.max_size);
        let cfg = Config::from_url(&uri);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create Redis connection pool");
        Self { pool }
    }
}

fn build_redis_uri(base: &str, db: &u32, timeout_seconds: &u32, max_size: &u32) -> String {
    // 可选：添加参数校验逻辑，防止非法值
    format!(
        "{}?db={}&timeout={}&max_connections={}",
        base, db, timeout_seconds, max_size
    )
}

#[cfg(test)]
mod tests {
    use deadpool_redis::redis::cmd;

    use super::*;

    #[tokio::test]
    async fn test_redis_set() {
        let redis = RedisInstance::default();
        let mut conn = redis.pool.get().await.unwrap();
        cmd("SET")
            .arg(&["deadpool/test_key", "42"])
            .query_async::<()>(&mut conn)
            .await.unwrap();
    }

    #[tokio::test]
    async fn test_redis_get() {
        let redis = RedisInstance::default();
        let mut conn = redis.pool.get().await.unwrap();
        let value: String = cmd("GET")
            .arg(&["deadpool/test_key"])
            .query_async(&mut conn)
            .await.unwrap();
        assert_eq!(value, "42".to_string());
    }
}