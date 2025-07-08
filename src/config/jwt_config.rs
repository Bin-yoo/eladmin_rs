use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JwtConfig {
    pub header: String,
    #[serde(rename = "token-start-with")]
    pub token_start_with: String,
    #[serde(rename = "base64-secret")]
    pub base64_secret: String,
    #[serde(rename = "token-validity-in-seconds")]
    pub token_validity_in_seconds: u64,
    #[serde(rename = "online-key")]
    pub online_key: String,
    #[serde(rename = "code-key")]
    pub code_key: String,
    pub detect: u64,
    pub renew: u64,
}