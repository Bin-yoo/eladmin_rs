use serde::{Deserialize, Serialize};

use crate::system::domain::entity::sys_role::DataScope;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleSmallDto {
    pub id: i64,
    pub name: String,
    pub level: i32,
    pub data_scope: DataScope,
}