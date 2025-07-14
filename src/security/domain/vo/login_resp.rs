use serde::Serialize;

use crate::security::domain::dto::jwt_user::JwtUserDTO;

#[derive(Debug, Serialize)]
pub struct LoginRespVo {
    /// token
    pub token: String,
    /// 
    pub user: JwtUserDTO,
}