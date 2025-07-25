use serde::{Deserialize, Serialize};

/// 构建前端路由时用到
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MenuVO {
    /// 菜单标题或者组件名称
    pub name: Option<String>,

    /// 菜单路由
    pub path: Option<String>,

    /// 隐藏
    pub hidden: Option<bool>,

    pub redirect: Option<String>,

    /// 组件
    pub component: Option<String>,

    pub always_show: bool,

    pub meta: Option<MenuMetaVo>,

    /// 子菜单
    pub children: Vec<MenuVO>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MenuMetaVo {
    /// 菜单标题
    pub title: Option<String>,

    /// 图标
    pub icon: Option<String>,

    // 是否缓存
    pub no_cache: bool,
}