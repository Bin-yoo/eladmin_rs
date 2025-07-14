use salvo::prelude::*;
use salvo::logging::Logger;
use tokio::signal;

use eladmin_rs::util;

#[tokio::main]
async fn main() {
    // 初始化一个默认的订阅器，将日志输出到控制台
    tracing_subscriber::fmt::init();

    // 创建监听器
    let ip = "127.0.0.1";
    let port = util::app_config::APP_CONFIG.webserver.port.as_str();
    let acceptor = TcpListener::new(format!("{}:{}", ip, port)).bind().await;

    // 数据库连接初始化
    util::db_connection::init_rbatis().await;
    
    // 初始化redis连接
    // 初始化
    util::redis::RedisInstance::init_redis();

    // 创建路由
    let router = util::routes::create_router();

    let service = Service::new(router)
        // 添加中间件
        // 日志中间件
        .hoop(Logger::new())
        // 跨域配置
        .hoop(util::app_config::get_cors_config());
    
    let server = Server::new(acceptor);
    // 获取服务器的句柄
    let handler = server.handle();

    // 监听信号以实现优雅停机
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
        tracing::info!("Received SIGINT/SIGTERM, shutting down gracefully...");
        handler.stop_graceful(std::time::Duration::from_secs(60));
    });

    // Start serving requests
    server.serve(service).await;
}
