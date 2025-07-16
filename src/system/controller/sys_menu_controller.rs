use salvo::{handler, writing::Json, Depot};
use anyhow::Result;

use crate::system::domain::dto::sys_user::SysUserDTO;
use crate::system::domain::vo::sys_menu::MenuVO;
use crate::system::service::sys_menu_service as menu_service;

/// 获取前端所需菜单
#[handler]
pub async fn build_menus(depot: &mut Depot) -> Result<Json<Vec<MenuVO>>> {
    let user_info = depot.obtain::<SysUserDTO>().unwrap();

    let menu_dto_vec = menu_service::find_by_user(&user_info.user_id).await?;
    let menu_dtos = menu_service::build_tree(menu_dto_vec);
    // tracing::debug!("menu_dtos: {:?}", serde_json::to_string(&menu_dtos).unwrap());
    let menu_vos = menu_service::build_menus(menu_dtos);
    Ok(Json(menu_vos))
}