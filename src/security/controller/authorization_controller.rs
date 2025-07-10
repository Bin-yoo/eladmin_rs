use salvo::{handler, writing::Json, Depot, Request, Response};
use uuid::Uuid;
use anyhow::{Ok, Result};

use crate::{security::domain::{dto::auth_user::AuthUserDTO, vo::code_result::CodeResultVO}};
use crate::{APP_CONFIG, RedisInstance, AppError};

use crate::util::rsa_utils;

#[handler]
pub async fn get_code() -> Result<Json<CodeResultVO>> {
    let login_properties = APP_CONFIG.login.clone();
    let jwt_properties = APP_CONFIG.jwt.clone();

    if !login_properties.login_code.enabled {
        let result = CodeResultVO {
            enabled: 0,
            img: None,
            uuid: None,
        };
        return Ok(Json(result));
    }

    // 创建验证码
    let captcha = login_properties.login_code.switch_captcha();

    // UUID
    let uuid = format!("{}{}", jwt_properties.code_key, Uuid::new_v4());

    // 存入 Redis
    let expiration = login_properties.login_code.expiration * 60;
    RedisInstance::set_with_expiration(&uuid, &captcha.text, expiration).await
        .map_err(|e| AppError::BadRequest("验证码生成失败".to_string(), e.to_string()))?;

    // 返回响应
    let result = CodeResultVO {
        enabled: 1,
        img: Some(captcha.to_base64()),
        uuid: Some(uuid),
    };

    Ok(Json(result))
}

pub async fn login(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    // 获取请求体
    let payload: AuthUserDTO = req.parse_body().await?;

    // 密码解密
    let password = rsa_utils::decrypt_by_private_key(APP_CONFIG.rsa.private_key.clone().as_str(), &payload.password)?;

    // 开启验证码校验
    if APP_CONFIG.login.login_code.enabled.clone() {
        if let Some(code) = RedisInstance::get::<String>(&payload.uuid).await? {
            if payload.code.is_none() || !code.eq_ignore_ascii_case(&payload.code.unwrap()) {
                AppError::BadRequest("验证码错误".to_string(), String::new());
            }
        } else {
            AppError::BadRequest("验证码不存在或已过期".to_string(), String::new());
        }
    }

    // 生成 JWT Token
    todo!("生成 JWT Token,返回登录结果");
}
