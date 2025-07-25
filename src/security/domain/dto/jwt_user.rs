use serde::{Deserialize, Serialize};

use crate::system::domain::dto::sys_user::SysUserDTO;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    /// UUID
    pub uuid: String,
    /// 用户名
    pub username: String,
    /// 过期时间
    pub exp: usize
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtUserDTO {
    /// 用户名
    pub user: SysUserDTO,
    /// 菜单/接口权限
    pub roles: Vec<String>,
    /// 数据权限
    pub data_scopes: Vec<i64>,
}
