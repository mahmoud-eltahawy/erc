use errc::{
    api::{
        department::sets_department_boss,
        employee::{down_employee, up_employee},
        permissions::{allow_permission, forbid_permission},
    },
    config::AppState,
    memory::{
        department::{find_all_departments, find_department_by_id},
        employee::{
            find_9_non_admins, find_9_non_admins_by_name, find_admins, find_department_8_employees,
            find_department_employees_by_name, find_employee_name_by_id,
            find_employees_by_department_id_except_boss,
        },
        permissions::{find_department_permissions_by_id, find_employee_permissions_by_id},
    },
    syncing::upgrade,
    translator::translate_permission,
};
use rec::model::{
    department::Department,
    name::Name,
    permissions::{PermissionName, Permissions},
};
use serde::{Deserialize, Serialize};
use tauri::Window;
use uuid::Uuid;

#[tauri::command]
pub async fn search_non_admins(
    app_state: tauri::State<'_, AppState>,
    name: Option<String>,
) -> Result<Vec<Name>, String> {
    if let Some(name) = name {
        match find_9_non_admins_by_name(&app_state.pool, &name.trim()).await {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_9_non_admins(&app_state.pool).await {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[tauri::command]
pub async fn search_admins(app_state: tauri::State<'_, AppState>) -> Result<Vec<Name>, String> {
    match find_admins(&app_state.pool).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn list_departments(app_state: tauri::State<'_, AppState>) -> Result<Vec<Name>, String> {
    match find_all_departments(&app_state.pool).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn department_employees(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Vec<Name>, String> {
    match find_employees_by_department_id_except_boss(&app_state.pool, &id).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn boss_employee(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
    window: Window,
) -> Result<(), String> {
    let Ok(_) = sets_department_boss(&app_state,&id).await else {
    return Err("".to_string())
  };

    let Ok(()) = upgrade(&app_state, Some(&window)).await else {
    return Err("".to_string());
  };

    Ok(())
}

#[tauri::command]
pub async fn find_department(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Department<String>, String> {
    match find_department_by_id(&app_state.pool, id.to_string()).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn employee_name(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<String, String> {
    match find_employee_name_by_id(&app_state.pool, id.to_string()).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClassifiedPermissions {
    id: String,
    allowed: Vec<(String, PermissionName)>,
    forbidden: Vec<(String, PermissionName)>,
}

#[tauri::command]
pub async fn department_permissions(
    app_state: tauri::State<'_, AppState>,
    department_id: Uuid,
) -> Result<ClassifiedPermissions, String> {
    match find_department_permissions_by_id(&app_state.pool, department_id.to_string()).await {
        Ok(permissions) => {
            let (allowed, forbidden) = permissions.list();
            let allowed: Vec<(String, PermissionName)> = allowed
                .into_iter()
                .map(|p| (translate_permission(&p), p))
                .collect();
            let forbidden: Vec<(String, PermissionName)> = forbidden
                .into_iter()
                .map(|p| (translate_permission(&p), p))
                .collect();
            Ok(ClassifiedPermissions {
                id: permissions.id,
                allowed,
                forbidden,
            })
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn employee_permissions_classified(
    app_state: tauri::State<'_, AppState>,
    employee_id: Uuid,
) -> Result<ClassifiedPermissions, String> {
    match find_employee_permissions_by_id(&app_state.pool, employee_id.to_string()).await {
        Ok(permissions) => {
            let (allowed, forbidden) = permissions.list();
            let allowed: Vec<(String, PermissionName)> = allowed
                .into_iter()
                .map(|p| (translate_permission(&p), p))
                .collect();
            let forbidden: Vec<(String, PermissionName)> = forbidden
                .into_iter()
                .map(|p| (translate_permission(&p), p))
                .collect();
            Ok(ClassifiedPermissions {
                id: permissions.id,
                allowed,
                forbidden,
            })
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn employee_permissions(
    app_state: tauri::State<'_, AppState>,
    id: Option<Uuid>,
) -> Result<Permissions<String>, String> {
    match id {
        Some(id) => match find_employee_permissions_by_id(&app_state.pool, id.to_string()).await {
            Ok(permissions) => Ok(permissions),
            Err(err) => Err(err.to_string()),
        },
        None => Ok(Permissions::default(Uuid::default()).string_to_client()),
    }
}

#[tauri::command]
pub async fn permission_allow(
    app_state: tauri::State<'_, AppState>,
    window: Window,
    id: Uuid,
    permission: PermissionName,
) -> Result<(), String> {
    let Ok(()) = allow_permission(&app_state, id, &permission).await else {
    return Err("".to_string());
  };
    let Ok(()) = upgrade(&app_state, Some(&window)).await else {
    return Err("".to_string());
  };
    Ok(())
}

#[tauri::command]
pub async fn permission_forbid(
    app_state: tauri::State<'_, AppState>,
    window: Window,
    id: Uuid,
    permission: PermissionName,
) -> Result<(), String> {
    let Ok(()) = forbid_permission(&app_state, id, &permission).await else {
    return Err("".to_string());
  };
    let Ok(()) = upgrade(&app_state, Some(&window)).await else {
    return Err("".to_string());
  };
    Ok(())
}

#[tauri::command]
pub async fn admin_employee(
    app_state: tauri::State<'_, AppState>,
    window: Window,
    id: Uuid,
) -> Result<(), String> {
    let Ok(()) = up_employee(&app_state,&id).await else {
    return Err("".to_string());
  };
    let Ok(()) = upgrade(&app_state, Some(&window)).await else {
    return Err("".to_string());
  };
    Ok(())
}

#[tauri::command]
pub async fn unadmin_employee(
    app_state: tauri::State<'_, AppState>,
    window: Window,
    id: Uuid,
) -> Result<(), String> {
    let Ok(()) = down_employee(&app_state,&id).await else {
    return Err("".to_string());
  };
    let Ok(()) = upgrade(&app_state, Some(&window)).await else {
    return Err("".to_string());
  };
    Ok(())
}

#[tauri::command]
pub async fn search_department_employees(
    app_state: tauri::State<'_, AppState>,
    name: Option<String>,
    department_id: Uuid,
) -> Result<Vec<Name>, String> {
    if let Some(name) = name {
        if name == "*" {
            return match find_employees_by_department_id_except_boss(
                &app_state.pool,
                &department_id,
            )
            .await
            {
                Ok(days) => Ok(days),
                Err(err) => Err(err.to_string()),
            };
        }
        match find_department_employees_by_name(&app_state.pool, &name.trim(), &department_id).await
        {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_department_8_employees(&app_state.pool, &department_id).await {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    }
}
