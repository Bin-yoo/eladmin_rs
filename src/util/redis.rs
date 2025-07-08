use std::sync::OnceLock;

use anyhow::{Ok, Result};
use deadpool_redis::{redis::{cmd, FromRedisValue, ToRedisArgs}, Config, Pool, PoolError, Runtime};

use crate::util::app_config::APP_CONFIG;

pub static REDIS_INSTANCE: OnceLock<RedisInstance> = OnceLock::new();

type DeadpoolInstance = Pool;

#[derive(Debug)]
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
        tracing::info!("[eladmin_rs] redis pool init success!");
        Self { pool }
    }

    /// 初始化 Redis 实例
    pub fn init_redis() {
        let instance = Self::new();
        REDIS_INSTANCE.set(instance).expect("Failed to initialize Redis instance");
    }

    // 获取实例的只读引用
    pub fn instance() -> &'static RedisInstance {
        REDIS_INSTANCE.get().expect("Redis instance not initialized")
    }

    /// 获取 Redis 连接
    ///
    /// 从连接池中获取一个可用的 Redis 连接。
    ///
    /// # 返回值
    /// - `Ok(Connection)`：成功获取到的 Redis 连接。
    /// - `Err(RedisError)`：连接失败时返回的错误信息。
    ///
    /// # 错误处理
    /// 可能的错误包括：
    /// - 连接池关闭 (`Connection pool is closed`)；
    /// - 连接超时 (`Connection pool timeout`)；
    /// - 后端错误 (`Backend error`)；
    /// - 未指定运行时 (`No runtime specified for the connection pool`)；
    /// - 创建连接后钩子失败 (`Post create hook failed`)。
    async fn get_connection() -> Result<deadpool_redis::Connection> {
        let conn = RedisInstance::instance()
        .pool
        .get()
        .await
        .map_err(|e| match e {
            PoolError::Closed => anyhow::anyhow!("Connection pool is closed"),
            PoolError::Timeout(_) => anyhow::anyhow!("Connection pool timeout"),
            PoolError::Backend(ref err) => anyhow::anyhow!("{}", err.to_string()),
            PoolError::NoRuntimeSpecified => anyhow::anyhow!("No runtime specified for the connection pool"),
            PoolError::PostCreateHook(_) => anyhow::anyhow!("Post create hook failed"),
        })?;
        Ok(conn)
    }

    /// 设置键值对缓存，并设置过期时间
    ///
    /// 将指定的键值对存储到 Redis 中，并设置过期时间。
    ///
    /// # 参数
    /// - `key`：Redis 中的键，字符串类型。
    /// - `value`：要存储的值，实现了 `ToRedisArgs` 的任意类型。
    /// - `expiration`：过期时间，单位为秒。
    ///
    /// # 返回值
    /// - `Ok(())`：操作成功。
    /// - `Err(RedisError)`：操作失败时返回的错误信息。
    ///
    /// # 示例
    /// ```rust
    /// use deadpool_redis::redis::cmd;
    ///
    /// let redis = RedisInstance::new();
    /// redis.set_with_expiration("test_key", "60", 60).await.unwrap();
    /// ```
    pub async fn set_with_expiration<T>(key: &str, value: T, expiration: u64) -> Result<()>
    where
        T: ToRedisArgs,
    {
        let mut conn = RedisInstance::get_connection().await?;

        let _ = cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX").arg(expiration)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| e);
        Ok(())
    }

    /// 设置键值对缓存
    ///
    /// 将指定的键值对存储到 Redis 中，不设置过期时间。
    ///
    /// # 参数
    /// - `key`：Redis 中的键，字符串类型。
    /// - `value`：要存储的值，实现了 `ToRedisArgs` 的任意类型。
    ///
    /// # 返回值
    /// - `Ok(())`：操作成功。
    /// - `Err(RedisError)`：操作失败时返回的错误信息。
    ///
    /// # 示例
    /// ```rust
    /// use deadpool_redis::redis::cmd;
    ///
    /// let redis = RedisInstance::new();
    /// redis.set("test_key", "42").await.unwrap();
    /// ```
    pub async fn set<T>(key: &str, value: T) -> Result<()>
    where
        T: ToRedisArgs,
    {
        let mut conn = RedisInstance::get_connection().await?;

        let _ = cmd("SET")
            .arg(key)
            .arg(value)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| e);
        Ok(())
    }

    /// 获取键对应的值
    ///
    /// 从 Redis 中获取指定键的值。
    ///
    /// # 参数
    /// - `key`：Redis 中的键，字符串切片类型。
    ///
    /// # 返回值
    /// - `Ok(Some(T))`：成功获取到的值，类型为实现了 `FromRedisValue` 的任意类型。
    /// - `Ok(None)`：键不存在时返回 `None`。
    /// - `Err(RedisError)`：操作失败时返回的错误信息。
    ///
    /// # 示例
    /// ```rust
    /// use deadpool_redis::redis::cmd;
    ///
    /// let redis = RedisInstance::new();
    /// let value = redis.get::<String>("test_key").await.unwrap();
    /// assert_eq!(value, Some("42".to_string()));
    /// ```
    pub async fn get<T>(key: &str) -> Result<Option<T>>
    where
        T: FromRedisValue,
    {
        let mut conn = RedisInstance::get_connection().await?;
        let value: Option<T> = cmd("GET").arg(key).query_async(&mut conn).await?;
        Ok(value)
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
        RedisInstance::init_redis();
        RedisInstance::set("test_key", "42").await.unwrap();
    }

    #[tokio::test]
    async fn test_redis_set_with_ttl() {
        RedisInstance::init_redis();
        RedisInstance::set_with_expiration("test_key_with_ttl", "60", 60).await.unwrap();
    }

    #[tokio::test]
    async fn test_redis_get() {
        RedisInstance::init_redis();
        let value: Option<String> = RedisInstance::get("test_key").await.unwrap();
        assert_eq!(value, Some("42".to_string()));
    }
}