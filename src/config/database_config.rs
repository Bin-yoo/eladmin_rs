use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u64,
    pub timeout_seconds: u64,
}