use serde::{Serialize, Deserialize};
use rbatis::{crud, rbdc::DateTime};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysRole {
    /// ID
    pub role_id: i64,

    /// 名称
    pub name: String,

    /// 角色级别
    pub level: i32,

    /// 描述
    pub description: Option<String>,

    /// 数据权限（"全部", "本级", "自定义"）
    pub data_scope: String,

    /// 创建者
    pub create_by: Option<String>,

    /// 更新者
    pub update_by: Option<String>,

    /// 创建日期
    pub create_time: Option<DateTime>,

    /// 更新时间
    pub update_time: Option<DateTime>,
}

crud!(SysRole{});

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataScopeEnum {
    /// 全部数据权限
    All,
    /// 本级及以下数据权限
    ThisLevel,
    /// 自定义数据权限
    Customize,
}

impl DataScopeEnum {
    pub fn value(&self) -> &'static str {
        match self {
            DataScopeEnum::All => "全部",
            DataScopeEnum::ThisLevel => "本级",
            DataScopeEnum::Customize => "自定义",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            DataScopeEnum::All => "全部的数据权限",
            DataScopeEnum::ThisLevel => "自己部门的数据权限",
            DataScopeEnum::Customize => "自定义的数据权限",
        }
    }

    pub fn find(value: &str) -> Option<Self> {
        match value {
            "全部" => Some(DataScopeEnum::All),
            "本级" => Some(DataScopeEnum::ThisLevel),
            "自定义" => Some(DataScopeEnum::Customize),
            _ => None,
        }
    }

    /// 获取所有枚举值列表，常用于下拉选项等前端展示场景
    pub fn values() -> Vec<&'static str> {
        vec!["全部", "本级", "自定义"]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleSmallDto {
    pub id: i64,
    pub name: String,
    pub level: i32,
    pub data_scope: String,
}