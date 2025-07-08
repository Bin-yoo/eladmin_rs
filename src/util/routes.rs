use salvo::prelude::*;

use crate::security::authorization_controller;

pub fn create_router() -> Router {
    Router::new()
        .hoop(CatchPanic::new())
        .get(hello)
        .push(Router::with_path("你好").get(hello_zh))
        // 添加其他路由
        .push(auth_router())
}

fn auth_router() -> Router {
    Router::new()
        .path("auth")
        .push(
            Router::with_path("code").get(authorization_controller::get_code)
        )
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