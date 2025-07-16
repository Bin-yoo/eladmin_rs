use rbatis::{executor::Executor, html_sql, impled};

use crate::system::domain::entity::sys_menu::SysMenu;

#[html_sql("src/system/mapper/sys_menu_mapper.html")]
pub async fn select_link_role(rb: & dyn Executor, role_ids: &[i64], m_type: i32) -> Vec<SysMenu> {
    impled!()
}

#[html_sql("src/system/mapper/sys_menu_mapper.html")]
pub async fn select_link(rb: & dyn Executor, role_ids: &[i64]) -> Vec<SysMenu> {
    impled!()
}
