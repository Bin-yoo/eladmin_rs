use std::collections::HashSet;

use rbs::value;
use anyhow::Result;

use crate::RB;
use crate::system::domain::dto::sys_user::SysUserDTO;
use crate::system::domain::dto::sys_role::RoleSmallDto;
use crate::system::service::sys_menu_service;

/// 通过用户ID查询角色
pub async fn find_by_user_id(id:  &i64) -> Result<Vec<RoleSmallDto>> {
    let sql = "SELECT r.role_id as id, r.* 
        FROM sys_role r 
        LEFT OUTER JOIN sys_users_roles ur ON r.role_id=ur.role_id 
        LEFT OUTER JOIN sys_user u ON ur.user_id=u.user_id WHERE u.user_id = ?";
    let roles:Vec<RoleSmallDto> = RB.query_decode(sql, vec![value!(id)]).await?;
    Ok(roles)
}

/// 为用户映射授予的权限
pub async fn map_to_granted_authorities(user: &SysUserDTO) -> Result<Vec<String>> {
    // 如果是管理员，直接返回 "admin" 权限
    if user.is_admin {
        return Ok(vec!["admin".to_string()]);
    }

    // 查询用户的角色
    let roles = find_by_user_id(&user.user_id).await?;
    let mut permissions: HashSet<String> = HashSet::new();

    if !roles.is_empty() {
        // 提取所有角色的 ID
        let role_ids: Vec<i64> = roles.iter().map(|role| role.id).collect();

        // 查询角色关联的菜单
        let menus = sys_menu_service::find_by_role_ids(&role_ids).await?;

        // 提取有效的权限
        permissions.extend(
            menus
                .iter()
                .filter_map(|menu| menu.permission.as_ref().filter(|&p| !p.is_empty()).cloned())
        );
    }

    Ok(permissions.into_iter().collect())
}