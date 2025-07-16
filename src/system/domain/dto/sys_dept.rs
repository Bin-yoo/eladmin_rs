use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeptDto {
    /// ID
    pub dept_id: i64,

    /// 上级部门ID
    pub pid: Option<i64>,

    /// 子部门数目
    pub sub_count: i32,

    /// 名称
    pub name: String,

    /// 排序
    pub dept_sort: i32,

    /// 状态 ("true"/"false")
    pub enabled: bool,

    // 子部门
    pub children: Vec<DeptDto>
}