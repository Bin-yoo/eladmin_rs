use std::fmt::{Debug, Display, Formatter};

use rbatis::{crud, rbdc::DateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysMenu {
    /// 菜单ID
    pub menu_id: i64,

    /// 上级菜单ID
    pub pid: Option<i64>,

    /// 子菜单数目
    pub sub_count: i32,

    /// 菜单类型
    #[serde(alias = "type")]
    pub type_: MenuType,

    /// 菜单标题
    pub title: Option<String>,

    /// 组件名称
    pub name: Option<String>,

    /// 组件
    pub component: Option<String>,

    /// 排序
    pub menu_sort: i32,

    /// 图标
    pub icon: Option<String>,

    /// 链接地址
    pub path: Option<String>,

    /// 是否外链
    pub i_frame: Option<bool>,

    /// 缓存
    pub cache: Option<bool>,

    /// 隐藏
    pub hidden: Option<bool>,

    /// 权限
    pub permission: Option<String>,

    /// 创建者
    pub create_by: Option<String>,

    /// 更新者
    pub update_by: Option<String>,

    /// 创建日期
    pub create_time: Option<DateTime>,

    /// 更新时间
    pub update_time: Option<DateTime>,
}

crud!(SysMenu{});

#[derive(Clone, PartialEq, Eq)]
pub enum MenuType {
    FOLDER,
    MENU,
    BUTTON
}

impl From<i32> for MenuType {
    fn from(value: i32) -> Self {
        match value {
            0 => MenuType::FOLDER,
            1 => MenuType::MENU,
            2 => MenuType::BUTTON,
            _ => panic!("Invalid menu type")
        }
    }
}

impl From<MenuType> for i32 {
    fn from(value: MenuType) -> Self {
        match value {
            MenuType::FOLDER => 0,
            MenuType::MENU => 1,
            MenuType::BUTTON => 2
        }
    }
}

impl Debug for MenuType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(<i32>::from(self.clone()).to_string().as_str())
    }
}

impl Display for MenuType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(<i32>::from(self.clone()).to_string().as_str())
    }
}

impl serde::Serialize for MenuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for MenuType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = i32::deserialize(deserializer)?;
        Ok(MenuType::from(v))
    }
}