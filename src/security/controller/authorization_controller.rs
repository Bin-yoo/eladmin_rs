use salvo::{handler, prelude::JwtAuthDepotExt, writing::Json, Depot, Request};
use uuid::Uuid;
use anyhow::Result;

use crate::security::domain::dto::{auth_user::AuthUserDTO, jwt_user::{JwtUserDTO, JwtClaims}};
use crate::security::domain::vo::{code_result::CodeResultVO, login_resp::LoginRespVo};
use crate::security::service::authorization_service;
use crate::system::domain::dto::sys_user::SysUserDTO;

use crate::{APP_CONFIG, RedisInstance, AppError};

use crate::util::rsa_utils;

/// 获取验证码
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
    RedisInstance::set_with_expiration(&uuid, &captcha.text, expiration).await?;

    // 返回响应
    let result = CodeResultVO {
        enabled: 1,
        img: Some(captcha.to_base64()),
        uuid: Some(uuid),
    };

    Ok(Json(result))
}

/// 登录
#[handler]
pub async fn login(req: &mut Request) -> Result<Json<LoginRespVo>, AppError> {
    // 获取请求体
    let payload: AuthUserDTO = req.parse_body().await?;

    // 密码解密
    let password = rsa_utils::decrypt_by_private_key(APP_CONFIG.rsa.private_key.clone().as_str(), &payload.password)?;

    // 开启验证码校验
    if APP_CONFIG.login.login_code.enabled.clone() {
        if let Some(code) = RedisInstance::get::<String>(&payload.uuid).await? {
            if payload.code.is_none() || !code.eq_ignore_ascii_case(&payload.code.unwrap()) {
                return Err(AppError::BadRequest("验证码错误".to_string()));
            }
        } else {
            return Err(AppError::BadRequest("验证码不存在或已过期".to_string()));
        }
    }

    // 账号密码认证
    let jwt_user = authorization_service::username_password_authentication(&payload.username, &password).await?;
    // 生成 JWT Token
    let token_id = uuid::Uuid::new_v4().to_string();
    let cache_key = format!("{}{}", APP_CONFIG.jwt.online_key.clone(), token_id);
    let token = format!("{} {}", APP_CONFIG.jwt.token_start_with.clone(), authorization_service::generate_jwt_token(&jwt_user, &token_id)?);
    // 缓存用户信息
    RedisInstance::set_with_expiration(&cache_key, &jwt_user.user, APP_CONFIG.jwt.token_validity_in_seconds / 1000).await?;

    // 删除验证码
    RedisInstance::delete(&payload.uuid).await?;
    // 构建返回结果
    Ok(Json(LoginRespVo{
        token,
        user: jwt_user
    }))
}

#[handler]
pub async fn get_user_info(depot: &mut Depot) -> Result<Json<JwtUserDTO>, AppError> {
    let user_info = depot.obtain::<SysUserDTO>().unwrap();
    Ok(Json(authorization_service::load_user_by_username(&user_info.username).await?))
}

#[handler]
pub async fn logout(depot: &mut Depot) -> Result<(), AppError> {
    let data = depot.jwt_auth_data::<JwtClaims>().unwrap();
    let cache_key = format!("{}{}", APP_CONFIG.jwt.online_key.clone(), data.claims.uuid);
    RedisInstance::delete(&cache_key).await?;
    Ok(())
}