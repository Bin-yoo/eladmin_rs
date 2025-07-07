use salvo::prelude::*;

use eladmin_rs::util::app_config::APP_CONFIG;
use eladmin_rs::util;

#[tokio::main]
async fn main() {
    // 初始化一个默认的订阅器，将日志输出到控制台
    tracing_subscriber::fmt::init();

    // 创建监听器
    let ip = "127.0.0.1";
    let port = APP_CONFIG.webserver.port.as_str();
    let acceptor = TcpListener::new(format!("{}:{}", ip, port)).bind().await;

    // 数据库连接初始化
    util::db_connection::init_rbatis().await;

    // 创建路由
    let router = Router::new()
        .hoop(CatchPanic::new())
        .get(hello)
        .push(Router::with_path("你好").get(hello_zh));
    
    let service = Service::new(router)
    // 跨域配置
        .hoop(util::app_config::get_cors_config());
    
    let server = Server::new(acceptor);
    // 获取服务器的句柄
    let handler = server.handle();
    // 优雅停机
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        handler.stop_graceful(None);
    });
    // Start serving requests
    server.serve(service).await;
}

// Handler for English greeting
#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

// Handler for Chinese greeting
#[handler]
async fn hello_zh() -> Result<&'static str, ()> {
    Ok("你好，世界！")
}