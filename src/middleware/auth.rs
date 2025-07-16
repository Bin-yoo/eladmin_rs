use chrono::{Utc, Duration};
use salvo::prelude::*;

use crate::{AppError, RedisInstance, APP_CONFIG};
use crate::{security::domain::dto::jwt_user::JwtClaims, system::domain::dto::sys_user::SysUserDTO};

#[handler]
pub async fn check_token(depot: &mut Depot) -> Result<(), AppError> {
    match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {
            let data = depot.jwt_auth_data::<JwtClaims>().unwrap();
            let cache_key = format!("{}{}", APP_CONFIG.jwt.online_key.clone(), data.claims.uuid);

            // 从 Redis 获取用户信息的过期时间
            if let Some(ttl) = RedisInstance::get_ttl(&cache_key).await? {
                if ttl <= 0 {
                    return Err(AppError::Unauthorized);
                }
                let expiration = Utc::now() + Duration::seconds(ttl);
                let now = Utc::now();
                // 续期检查时间范围
                let detect_duration = APP_CONFIG.jwt.detect.clone();
                let differ = expiration.timestamp() - now.timestamp();
                // 判断是否在续期检查时间范围内
                if differ <= detect_duration {
                    // 续期
                    let renew = ttl * 1000 + APP_CONFIG.jwt.renew.clone();
                    RedisInstance::expire(&cache_key, renew).await?;
                }
            } else {
                return Err(AppError::Unauthorized);
            }

            // 获取用户信息
            if let Some(user_info) = RedisInstance::get::<SysUserDTO>(&cache_key).await?
            {
                // 注入用户信息,让后续的中间件可以获取用户信息
                tracing::debug!("用户信息: {:?}", &user_info);
                let _ = &depot.inject(user_info);
            } else {
                return Err(AppError::Unauthorized);
            }
        }
        JwtAuthState::Unauthorized => {
            return Err(AppError::Unauthorized);
        }
        JwtAuthState::Forbidden => {
            return Err(AppError::Forbidden);
        }
    }
    Ok(())
}