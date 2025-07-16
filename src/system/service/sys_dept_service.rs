use std::collections::HashSet;

use rbs::value;
use anyhow::{Ok, Result};

use crate::RB;
use crate::system::domain::entity::sys_dept::SysDept;
use crate::system::domain::dto::sys_dept::DeptDto;


/// 通过角色ID查询部门
pub async  fn find_by_role_id(role_id: &i64) -> Result<Vec<DeptDto>> {
    let sql = "SELECT d.dept_id as id, d.* 
        FROM sys_dept d LEFT OUTER JOIN sys_roles_depts rd ON d.dept_id=rd.dept_id WHERE rd.role_id = ?";
    let depts:Vec<DeptDto> = RB.query_decode(sql, vec![value!(role_id)])
        .await?;
    Ok(depts)
}

/// 通过pid查询部门
pub async fn find_by_pid(id: &i64) -> Result<Vec<SysDept>> {
    let depts: Vec<SysDept> = SysDept::select_by_map(&RB.clone(), value!("pid", id)).await?;
    if depts.is_empty() {
        return Ok(Vec::new());
    };
    Ok(depts)
}

/// 获取子部门
pub async fn get_dept_children(children: &[SysDept]) -> Result<Vec<i64>> {
    let mut result = HashSet::new();

    for child in children {
        if child.enabled {
            let sub_children = find_by_pid(&child.dept_id).await?;
            let child_ids = Box::pin(get_dept_children(&sub_children)).await?;
            if !child_ids.is_empty() {
                result.extend(child_ids.iter());
            }
            result.insert(child.dept_id.clone());
        }
    }

    Ok(result.into_iter().collect())
}