use std::sync::LazyLock;
use deadpool_redis::{redis::{cmd, ErrorKind, RedisError}, Config, Pool, PoolError, Runtime};

use crate::util::app_config::APP_CONFIG;

pub static REDIS_INSTANCE: LazyLock<RedisInstance> = LazyLock::new(|| RedisInstance::new());

type DeadpoolInstance = Pool;

pub struct RedisInstance {
    pub pool: DeadpoolInstance
}

impl RedisInstance {
    fn new() -> Self {
        let redis_cfg = APP_CONFIG.redis.clone();
        let uri = build_redis_uri(
            &redis_cfg.url,
            &redis_cfg.database,
            &redis_cfg.timeout_seconds,
            &redis_cfg.max_size,
        );
        let cfg = Config::from_url(&uri);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create Redis connection pool");
        Self { pool }
    }

    /// 获取 Redis 连接
    async fn get_connection(&self) -> Result<deadpool_redis::Connection, RedisError> {
        self.pool.get().await.map_err(|e| match e {
            PoolError::Closed => RedisError::from((ErrorKind::IoError, "Connection pool is closed")),
            PoolError::Timeout(_) => RedisError::from((ErrorKind::IoError, "Connection pool timeout")),
            PoolError::Backend(redis_error) => redis_error,
            PoolError::NoRuntimeSpecified => RedisError::from((ErrorKind::IoError, "No runtime specified for the connection pool")),
            PoolError::PostCreateHook(_) => RedisError::from((ErrorKind::IoError, "Post create hook failed"))
        })
    }

    /// 设置键值对
    /// key: 键
    /// value: 值
    /// expiration: 过期时间
    /// 返回值：Ok(()) 表示成功
    /// 错误处理：返回错误信息
    /// 示例：
    /// ```
    /// use deadpool_redis::redis::cmd;
    ///
    /// let redis = RedisInstance::new();
    /// redis.set("deadpool/test_key", "42", 60).await.unwrap();
    /// ```
    pub async fn set(&self, key: &str, value: &str, expiration: u64) -> Result<(), RedisError> {
        let mut conn = self.get_connection().await?;

        cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX").arg(expiration)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, deadpool_redis::redis::RedisError> {
        let mut conn = self.get_connection().await?;
        let value: String = cmd("GET")
            .arg(&[key])
            .query_async(&mut conn)
            .await?;
        Ok(Some(value))
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
    use super::*;

    #[tokio::test]
    async fn test_redis_set() {
        let redis = RedisInstance::new();
        redis.set("test_key", "42", 60).await.unwrap();
    }

    #[tokio::test]
    async fn test_redis_get() {
        let redis = RedisInstance::new();
        let value = redis.get("test_key").await.unwrap();
        assert_eq!(value, Some("42".to_string()));
    }
}