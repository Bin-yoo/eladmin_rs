use std::{env, fs, sync::LazyLock};
use salvo::{cors::{Cors, CorsHandler}, http::Method};
use config::{Config, File};
use serde::{Deserialize, Serialize};

use crate::config::{database_config, jwt_config, login_config, redis_config, rsa_config, web_server_config};

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::default());

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: database_config::DatabaseConfig,
    pub webserver: web_server_config::WebServerConfig,
    pub rsa: rsa_config::RsaConfig,
    pub redis: redis_config::RedisConfig,
    pub login: login_config::LoginProperties,
    pub jwt: jwt_config::JwtConfig
}

impl AppConfig {
    fn load_yml_config() -> AppConfig {
        let config_mode = env::var("CONFIG")
            .map(|s| if s.is_empty() { "dev".to_string() } else { s })
            .unwrap_or("dev".to_string());
        // 获取当前工作目录（适用于开发环境）
        let current_dir = env::current_dir().expect("无法获取当前工作目录");

        // 构建配置文件路径
        let config_file = match config_mode.as_str() {
            "release" => "config-release.yaml",
            _ => "config-dev.yaml",
        };
        let config_path = current_dir.join("config").join(config_file);
        tracing::info!("config_path========== {:?}", config_path);

        // 检查文件是否存在，避免 Config::builder 静默失败
        if !fs::metadata(&config_path).map_or(false, |m| m.is_file()) {
            panic!("Config file does not exist or is not a regular file: {}", config_file);
        }
        
        let settings = Config::builder()
            .add_source(File::from(config_path))
            .build()
            .expect("Could not load config file");

        let app_config = settings.try_deserialize::<AppConfig>()
            .expect("Failed to deserialize config");

        tracing::info!("Using config file: {}", config_file);
        app_config
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig::load_yml_config()
    }
}

/// CORS Config
/// https://salvo.rs/guide/cors.html
/// 跨域配置
pub fn get_cors_config() -> CorsHandler {
    Cors::new()
        .allow_origin("*")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT, Method::PATCH])
        .allow_headers("authorization")
        .into_handler()
}