use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::RB;
use crate::system::domain::vo::sys_menu::{MenuMetaVo, MenuVO};
use crate::system::domain::entity::sys_menu::{MenuType, SysMenu};
use crate::system::domain::dto::sys_menu::MenuDTO;
use crate::system::service::sys_role_service as role_service;
use crate::system::mapper::sys_menu_mapper;

/// 根据角色ID列表查询角色与菜单的映射关系
pub async fn find_by_role_ids(role_ids: &[i64]) -> Result<Vec<SysMenu>> {
    let role_menu_mapping: Vec<SysMenu> = sys_menu_mapper::select_link(&RB.clone(), role_ids).await?;
    Ok(role_menu_mapping)
}

/// 根据用户ID查询菜单
pub async fn find_by_user(user_id: &i64) -> Result<Vec<MenuDTO>> {
    let roles = role_service::find_by_user_id(user_id).await?;
    let role_ids: Vec<i64> = roles.iter().map(|role| role.id).collect();
    // 查询类型不为2(按钮)的菜单
    let menus = sys_menu_mapper::select_link_role(&RB.clone(), &role_ids, MenuType::BUTTON.into()).await?;
    Ok(menus.into_iter().map(MenuDTO::from).collect())
}

/// 构建菜单树
pub fn build_tree(menu_dtos: Vec<MenuDTO>) -> Vec<MenuDTO> {
    if menu_dtos.is_empty() {
        return vec![];
    }

    // Step 1: 创建 deep copy 的节点 map
    let mut id_map: HashMap<i64, MenuDTO> = HashMap::new();
    for dto in &menu_dtos {
        id_map.insert(dto.id, dto.clone());
    }

    // Step 2: 构建父子关系
    for dto in &menu_dtos {
        if let Some(pid) = dto.pid {
            if let Some(parent) = id_map.get_mut(&pid) {
                // 把当前节点插入到父节点的 children 中
                let current_node = dto.clone();
                parent.children.push(current_node);
            }
        }
    }

    // Step 3: 找出所有根节点（pid == None）
    let mut roots: Vec<MenuDTO> = menu_dtos
        .iter()
        .filter(|dto| dto.pid.is_none())
        .cloned()
        .collect();

    // 如果没有显式根节点，则 fallback 到非子节点
    if roots.is_empty() {
        let child_ids: HashSet<i64> = menu_dtos
            .iter()
            .filter_map(|dto| dto.pid)
            .collect();

        roots = menu_dtos
            .into_iter()
            .filter(|dto| !child_ids.contains(&dto.id))
            .collect();
    }

    // Step 4: 从 map 中提取完整结构
    roots
        .into_iter()
        .filter_map(|root| id_map.get(&root.id).cloned())
        .collect()
}

/// 构建菜单
pub fn build_menus(menu_dtos: Vec<MenuDTO>) -> Vec<MenuVO> {
    let mut vos: Vec<MenuVO> = Vec::new();
    
    for menu_dto in menu_dtos {
        let mut menu_vo = MenuVO::default();
        // 
        menu_vo.name = if menu_dto.component_name.is_none() {
            menu_dto.title.clone()
        } else {
            menu_dto.component_name.clone()
        };
        // 一级目录需要加斜杠，不然会报警告
        menu_vo.path = if menu_dto.pid.is_none() {
            Some(format!("/{}", menu_dto.path.clone().unwrap()))
        } else {
            menu_dto.path.clone()
        };
        menu_vo.hidden = menu_dto.hidden;

        // 如果不是外链
        let is_frame = menu_dto.i_frame.is_some() && !menu_dto.i_frame.unwrap();
        menu_vo.component = if is_frame {
            if menu_dto.pid.is_none() {
                if menu_dto.component.is_none() || menu_dto.component.as_ref().unwrap().is_empty() {
                    Some("Layout".to_string())
                } else {
                    menu_dto.component.clone()
                }
            } else if menu_dto.type_ == MenuType::FOLDER {
                if menu_dto.component.is_none() || menu_dto.component.as_ref().unwrap().is_empty() {
                    Some("ParentView".to_string())
                } else {
                    menu_dto.component.clone()
                }
            } else if menu_dto.component.is_some() && !menu_dto.component.as_ref().unwrap().is_empty() {
                menu_dto.component.clone()
            } else {
                None
            }
        } else {
            None
        };

        menu_vo.meta = Some(MenuMetaVo {
            icon: menu_dto.icon.clone(),
            no_cache: !menu_dto.cache.unwrap_or(false),
            title: menu_dto.title.clone()
        });
        
        if !menu_dto.children.is_empty() {
            menu_vo.always_show = true;
            menu_vo.redirect = Some("noredirect".to_string());
            menu_vo.children = build_menus(menu_dto.children);
        }
        // 处理是一级菜单并且没有子菜单的情况
        else if menu_dto.pid.is_none(){
            let mut menu_vo1 = MenuVO::default();
            menu_vo1.meta = menu_vo.meta.clone();
            if is_frame {
                menu_vo1.path = Some("index".to_string());
                menu_vo1.name = menu_vo.name.clone();
                menu_vo1.component = menu_vo.component.clone();
            } else {
                menu_vo1.path = menu_dto.path.clone();
            }
            menu_vo.name = None;
            menu_vo.meta = None;
            menu_vo.component = Some("Layout".to_string());
            menu_vo.children = vec![menu_vo1];
        }
        vos.push(menu_vo);
    }
    vos
}