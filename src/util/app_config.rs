use std::{env, sync::LazyLock};
use salvo::{cors::{Cors, CorsHandler}, http::Method};

use config::{Config, File};
use serde::{Deserialize, Serialize};

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::default());

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub webserver:  WebServerConfig,
    pub rsa: RsaConfig
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebServerConfig {
    pub port: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u64,
    pub timeout_seconds: u64
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RsaConfig {
    pub private_key: String
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