use rbs::value;
use anyhow::Result;

use crate::{security::domain::entity::sys_menu::SysMenu, RB};

/// 根据角色ID列表查询角色与菜单的映射关系
pub async fn find_by_role_ids(role_ids: &[i64]) -> Result<Vec<SysMenu>> {
    // 查询角色与菜单的映射关系
    let sql = "SELECT m.*
               FROM sys_roles_menus rm
               INNER JOIN sys_menu m ON rm.menu_id = m.menu_id
               WHERE rm.role_id IN (?)";
    let role_menu_mapping: Vec<SysMenu> = RB.query_decode(sql, vec![value!(role_ids)]).await?;
    Ok(role_menu_mapping)
}