use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedisConfig {
    pub url: String,
    pub database: u32,
    pub timeout_seconds: u32,
    pub max_size: u32,
}