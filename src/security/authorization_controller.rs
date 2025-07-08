use salvo::{handler, http::StatusCode, writing::Json, Response};
use serde::Serialize;
use uuid::Uuid;

use crate::util::{app_config::APP_CONFIG, redis::{RedisInstance}};

#[derive(Serialize)]
struct ImageResult {
    enabled: u8,
    img: Option<String>,
    uuid: Option<String>,
}



#[handler]
pub async fn get_code(res: &mut Response) {
    let login_properties = APP_CONFIG.login.clone();
    let jwt_properties = APP_CONFIG.jwt.clone();

    if !login_properties.login_code.enabled {
        let result = ImageResult {
            enabled: 0,
            img: None,
            uuid: None,
        };
        res.render(Json(result));
        return;
    }

    // 创建验证码
    let captcha = login_properties.login_code.switch_captcha();

    // UUID
    let uuid = format!("{}{}", jwt_properties.code_key, Uuid::new_v4());

    // 存入 Redis
    let expiration = login_properties.login_code.expiration * 60;
    match RedisInstance::set_with_expiration(&uuid, &captcha.text, expiration).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Failed to store captcha in Redis: {}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render("Failed to store captcha in Redis");
            return;
        }
    }

    // 返回响应
    let result = ImageResult {
        enabled: 1,
        img: Some(captcha.to_base64()),
        uuid: Some(uuid),
    };

    res.render(Json(result));
}
