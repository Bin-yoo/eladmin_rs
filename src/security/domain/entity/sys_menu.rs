use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysMenu {
    /// 菜单ID
    pub menu_id: i64,

    /// 上级菜单ID
    pub pid: Option<i64>,

    /// 子菜单数目
    pub sub_count: i32,

    /// 菜单类型
    pub type_: i32,

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
    pub i_frame: Option<String>,

    /// 缓存
    pub cache: Option<String>,

    /// 隐藏
    pub hidden: Option<String>,

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