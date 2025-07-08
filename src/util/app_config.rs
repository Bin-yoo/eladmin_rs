use std::{env, sync::LazyLock};
use salvo::{cors::{Cors, CorsHandler}, http::Method};
use config::{Config, File};
use serde::{Deserialize, Serialize};

use crate::config::{database_config, jwt_config, login_properties, redis_config, rsa_config, web_server_config};

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::default());

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: database_config::DatabaseConfig,
    pub webserver: web_server_config::WebServerConfig,
    pub rsa: rsa_config::RsaConfig,
    pub redis: redis_config::RedisConfig,
    pub login: login_properties::LoginProperties,
    pub jwt: jwt_config::JwtConfig
}

impl AppConfig {
    fn load_yml_config() -> AppConfig {
        let config_mode = env::var("CONFIG").unwrap_or("dev".to_string());
        let config_file = match config_mode.as_str() {
            "release" => "config/config-release.yaml",
            _ => "config/config-dev.yaml",
        };
        
        let settings = Config::builder()
            .add_source(File::with_name(config_file))
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