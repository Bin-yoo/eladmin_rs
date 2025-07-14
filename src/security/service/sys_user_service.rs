use rbs::value;

use crate::{security::domain::{dto::sys_user::SysUserDTO, entity::{sys_dept::SysDept, sys_user::SysUser}}, RB};


/// 通过用户名查询用户表
/// # 参数
/// - `username`：用户名
/// # 返回值
/// - `Ok(Some(SysUserDTO))`：操作成功。
/// - `Err(err)`：操作失败时返回的错误信息。
pub async fn find_by_name(username: &str) -> anyhow::Result<Option<SysUserDTO>> {
    // 查询用户表
    let Some(user) = SysUser::select_by_map(&RB.clone(), value! {"username": username})
                .await?
                .into_iter()
                .next()
    else {
        return Ok(None);
    };

    // 转换为 DTO
    let mut sys_user_dto = SysUserDTO::from(user.clone());

    // 查部门信息
    if let Some(dept) = SysDept::select_by_map(&RB.clone(), value! {"dept_id": user.dept_id})
                .await?
                .into_iter()
                .next()
    {
        sys_user_dto.set_dept(dept.dept_id, &dept.name);
    }
    Ok(Some(sys_user_dto))
}