use std::collections::HashSet;

use chrono::Utc;
use anyhow::Result;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{AppError, APP_CONFIG};
use crate::system::domain::{dto::sys_user::SysUserDTO, entity::sys_role::DataScopeEnum};
use crate::system::service::{sys_dept_service, sys_role_service, sys_user_service};
use crate::security::domain::dto::jwt_user::{JwtClaims, JwtUserDTO};

/// 账号密码认证
pub async fn username_password_authentication(username: &str, password: &str) -> Result<JwtUserDTO, AppError> {
    let jwt_user = load_user_by_username(username).await?;
    // 验证密码
    if !verify_password(&jwt_user.user.password, password) {
        return Err(AppError::BadRequest("用户名或密码错误".to_string()));
    }
    Ok(jwt_user)
}

/// 校验密码
fn verify_password(db_pass: &str, input_pass: &str) -> bool {
    bcrypt::verify(input_pass, db_pass).is_ok()
}

/// 通过用户名查询用户表
async fn load_user_by_username(username: &str) -> Result<JwtUserDTO, AppError> {
    let Some(user) = sys_user_service::find_by_name(username).await?
    else {
        return Err(AppError::UsernameNotFound);
    };

    if !user.enabled {
        return Err(AppError::BadRequest("账号未激活！".to_string()));
    }

    let jwt_user_dto = JwtUserDTO {
        user: user.clone(),
        roles: sys_role_service::map_to_granted_authorities(&user).await?,
        data_scopes: get_dept_ids(&user).await?
    };
    Ok(jwt_user_dto)
}

/// 获取数据权限的部门ID列表
async fn get_dept_ids(user: &SysUserDTO) -> Result<Vec<i64>> {
    // 查询用户角色
    let roles = sys_role_service::find_by_user_id(&user.user_id).await?;
    if !roles.is_empty() {
        // 存储部门ID
        let mut dept_ids: HashSet<i64> = HashSet::new();
        for role in roles {
            if let Some(data_scope_enum) = DataScopeEnum::find(&role.data_scope)
            {
                match data_scope_enum {
                    DataScopeEnum::ThisLevel => match &user.dept {
                        Some(dept) => {
                            dept_ids.insert(dept.id.clone());
                        },
                        None => {}
                    },
                    DataScopeEnum::Customize => {
                        get_customize(&mut dept_ids, role.id).await?;
                    },
                    _ => break
                }
            } else {
                return Err(anyhow::anyhow!("数据权限异常"));
            }
        }
        let dept_ids = Vec::from_iter(dept_ids);
        return Ok(dept_ids);
    }
    Ok(Vec::new())
}

/// 获取自定义的数据权限deptIds
async fn get_customize(
    dept_ids: &mut HashSet<i64>,
    role_id: i64,
) -> Result<()> {
    // 查询角色关联的部门
    let depts = sys_dept_service::find_by_role_id(&role_id).await?;

    for dept in depts {
        dept_ids.insert(dept.dept_id.clone());

        // 查询子部门
        let children = sys_dept_service::find_by_pid(&dept.dept_id).await?;
        if !children.is_empty() {
            let child_ids = sys_dept_service::get_dept_children(&children).await?;
            dept_ids.extend(child_ids);
        }
    }
    Ok(())
}

/// 生成JWT Token
pub fn generate_jwt_token(jwt_user_dto: &JwtUserDTO) -> Result<String> {
    let jwt_config = APP_CONFIG.jwt.clone();
    let expiration = Utc::now() + chrono::Duration::days(jwt_config.token_validity_in_seconds as i64);
    let claims = JwtClaims {
        exp: expiration.timestamp() as usize,
        uuid: uuid::Uuid::new_v4().to_string(),
        username: jwt_user_dto.user.username.clone(),
    };

    Ok(encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_config.base64_secret.as_bytes()))?)
}