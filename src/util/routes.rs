use salvo::prelude::*;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder};

use crate::middleware::auth::check_token;
use crate::security::controller::authorization_controller;
use crate::system::controller::*;
use crate::security::domain::dto::jwt_user::JwtClaims;
use crate::{AppError, APP_CONFIG};

pub fn create_router() -> Router {
    Router::new()
        .hoop(CatchPanic::new())
        .get(hello)
        .push(Router::with_path("你好").hoop(jwt_auth_handler()).get(hello_zh))
        // 添加其他路由
        .push(auth_router())
        .push(menu_router())
}

// Handler for English greeting
#[handler]
async fn hello() -> &'static str {
    "Backend service started successfully"
}

// Handler for Chinese greeting
#[handler]
async fn hello_zh(depot: &mut Depot) -> Result<&'static str, AppError> {
    match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {
            let data = depot.jwt_auth_data::<JwtClaims>().unwrap();
            println!("JwtClaims: {:?}", data);
            return Ok("你好，世界！");
        }
        JwtAuthState::Unauthorized => {
            return Err(AppError::Unauthorized);
        }
        JwtAuthState::Forbidden => {
            return Err(AppError::Forbidden);
        }
    }
}

/// jwt 认证
fn jwt_auth_handler() -> JwtAuth<JwtClaims, ConstDecoder> {
    let secret = APP_CONFIG.jwt.clone().base64_secret;
    JwtAuth::new(ConstDecoder::from_secret(secret.as_bytes()))
            .finders(vec![
                Box::new(HeaderFinder::new()),
                // Box::new(QueryFinder::new("jwt_token")),
                // Box::new(CookieFinder::new("jwt_token")),
            ])
            .force_passed(true)
}

/// 系统：系统授权接口
fn auth_router() -> Router {
    Router::new()
        .path("auth")
        .push(Router::with_path("code").get(authorization_controller::get_code))
        .push(Router::with_path("login").post(authorization_controller::login))
        .push(Router::with_path("info")
            .hoop(jwt_auth_handler())
            .hoop(check_token)
            .get(authorization_controller::get_user_info)
        )
        .push(Router::with_path("logout")
            .hoop(jwt_auth_handler())
            .delete(authorization_controller::logout)
        )
}

/// 系统：菜单管理
fn menu_router() -> Router {
    Router::new()
        .path("api")
        .path("menus")
        .hoop(jwt_auth_handler())
        .hoop(check_token)
        .push(Router::with_path("build").get(sys_menu_controller::build_menus))
}