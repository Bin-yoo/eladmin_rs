use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

use crate::system::domain::entity::sys_menu::{MenuType, SysMenu};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuDTO {
    /// 菜单ID
    pub id: i64,

    /// 上级菜单ID
    pub pid: Option<i64>,

    /// 子菜单
    pub children: Vec<MenuDTO>,

    /// 子菜单数目
    pub sub_count: i32,

    /// 菜单类型
    pub type_: MenuType,

    /// 菜单标题
    pub title: Option<String>,

    /// 组件名称
    pub component_name: Option<String>,

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

impl From<SysMenu> for MenuDTO {
    fn from(value: SysMenu) -> Self {
        Self {
            id: value.menu_id,
            pid: value.pid,
            children: Vec::new(),
            sub_count: value.sub_count,
            type_: value.type_,
            title: value.title,
            component_name: value.name,
            component: value.component,
            menu_sort: value.menu_sort,
            icon: value.icon,
            path: value.path,
            i_frame: value.i_frame,
            cache: value.cache,
            hidden: value.hidden,
            permission: value.permission,
            create_by: value.create_by,
            create_time: value.create_time,
            update_by: value.update_by,
            update_time: value.update_time
        }
    }
}
