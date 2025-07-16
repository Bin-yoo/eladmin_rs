use deadpool_redis::redis::{from_redis_value, FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use std::option::Option;

use crate::system::domain::entity::sys_user::SysUser;

/// 用户DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserDTO {
    /// 用户ID
    pub user_id: i64,

    /// 部门ID
    pub dept_id: Option<i64>,

    /// 部门
    pub dept: Option<DeptSmallDto>,

    /// 用户名
    pub username: String,

    /// 密码
    #[serde(skip_serializing, skip_deserializing)]
    pub password: String,

    /// 昵称
    pub nick_name: Option<String>,

    /// 性别
    pub gender: Option<String>,

    /// 手机号码
    pub phone: Option<String>,

    /// 邮箱
    pub email: Option<String>,

    /// 头像名称
    pub avatar_name: Option<String>,

    /// 头像路径
    pub avatar_path: Option<String>,

    /// 是否为 admin 账号
    pub is_admin: bool,

    /// 状态：1启用、0禁用
    pub enabled: bool,
}

impl SysUserDTO {
    pub fn set_dept(&mut self, dept_id: i64, dept_name: &str) {
        self.dept = Some(DeptSmallDto {
            id: dept_id,
            name: dept_name.to_owned(),
        });
    }
}

impl From<SysUser> for SysUserDTO {
    fn from(user: SysUser) -> Self {
        SysUserDTO {
            user_id: user.user_id,
            dept_id: user.dept_id,
            dept: None,
            username: user.username,
            password: user.password,
            nick_name: user.nick_name,
            gender: user.gender,
            phone: user.phone,
            email: user.email,
            avatar_name: user.avatar_name,
            avatar_path: user.avatar_path,
            is_admin: user.is_admin,
            enabled: user.enabled
        }
    }
}

impl ToRedisArgs for SysUserDTO {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + deadpool_redis::redis::RedisWrite
    {
        // 序列化存到redis中
        out.write_arg(serde_json::to_string(self).unwrap().as_bytes());
    }
}

impl FromRedisValue for SysUserDTO {
    fn from_redis_value(v: &deadpool_redis::redis::Value) -> deadpool_redis::redis::RedisResult<Self> {
        let v: String = from_redis_value(v)?;
        serde_json::from_slice::<SysUserDTO>(v.as_bytes())
            .map_err(deadpool_redis::redis::RedisError::from)
    }
}

/// Small部门结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeptSmallDto {
    /// 部门id
    pub id: i64,
    /// 部门名称
    pub name: String,
}