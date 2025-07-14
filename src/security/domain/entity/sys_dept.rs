use rbatis::{crud, rbdc::DateTime};
use serde::{Deserialize, Serialize};

// use crate::util::deserialize::deserialize_num_to_bool;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysDept {
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
    // #[serde(deserialize_with = "deserialize_num_to_bool")]
    pub enabled: bool,

    /// 创建者
    pub create_by: Option<String>,

    /// 更新者
    pub update_by: Option<String>,

    /// 创建时间
    pub create_time: Option<DateTime>,

    /// 更新时间
    pub update_time: Option<DateTime>,
}

crud!(SysDept {});

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