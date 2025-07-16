use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use rbatis::{crud, rbdc::DateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub data_scope: DataScope,

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

#[derive(Clone)]
pub enum DataScope {
    /// 全部数据权限
    All,
    /// 本级及以下数据权限
    ThisLevel,
    /// 自定义数据权限
    Customize,
}

impl From<&str> for DataScope {
    fn from(value: &str) -> Self {
        match value {
            "全部" => DataScope::All,
            "本级" => DataScope::ThisLevel,
            "自定义" => DataScope::Customize,
            _ => DataScope::All
        }
    }
}

impl From<DataScope> for &str {
    fn from(value: DataScope) -> Self {
        match value {
            DataScope::All => "全部",
            DataScope::ThisLevel => "本级",
            DataScope::Customize => "自定义",
        }
    }
}

impl Debug for DataScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(<&str>::from(self.clone()))
    }
}

impl Display for DataScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(<&str>::from(self.clone()))
    }
}

impl serde::Serialize for DataScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for DataScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = String::deserialize(deserializer)?;
        Ok(DataScope::from(v.as_str()))
    }
}
