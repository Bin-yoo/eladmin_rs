use serde::{Deserialize, Serialize};
use std::option::Option;
use rbatis::{crud, rbdc::DateTime};

// use crate::util::deserialize::deserialize_num_to_bool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysUser {
    /// 用户ID
    pub user_id: i64,

    /// 部门ID
    pub dept_id: Option<i64>,

    /// 用户名
    pub username: String,

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

    /// 密码
    pub password: String,

    /// 是否为 admin 账号
    // #[serde(deserialize_with = "deserialize_num_to_bool")]
    pub is_admin: bool,

    /// 状态：1启用、0禁用
    // #[serde(deserialize_with = "deserialize_num_to_bool")]
    pub enabled: bool,

    /// 创建者
    pub create_by: Option<String>,

    /// 更新者
    pub update_by: Option<String>,

    /// 修改密码时间
    pub pwd_reset_time: Option<DateTime>,

    /// 创建时间
    pub create_time: Option<DateTime>,

    /// 更新时间
    pub update_time: Option<DateTime>,
}

crud!(SysUser {});