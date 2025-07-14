use salvo::prelude::*;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder};

use crate::security::controller::authorization_controller;
use crate::security::domain::dto::jwt_user::JwtClaims;
use crate::{AppError, APP_CONFIG};

pub fn create_router() -> Router {
    Router::new()
        .hoop(CatchPanic::new())
        .get(hello)
        .push(Router::with_path("你好").hoop(jwt_auth_handler()).get(hello_zh))
        // 添加其他路由
        .push(auth_router())
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

fn auth_router() -> Router {
    Router::new()
        .path("auth")
        .push(Router::with_path("code").get(authorization_controller::get_code))
        .push(Router::with_path("login").post(authorization_controller::login))
}

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