use std::sync::Mutex;

use chrono::{Local, NaiveDateTime};
use erc_ui::{
    api::main_entry,
    config::AppState,
    memory::{
        department::{
            find_all_departments, find_department_boss_id_by_id, find_department_by_id,
            find_employee_department_id_and_boss_id,
        },
        employee::{
            find_9_non_admins, find_9_non_admins_by_name, find_admins, find_department_8_employees,
            find_department_employees_by_name, find_employee_name_by_id,
            find_employees_by_department_id_except_boss, find_employee_by_id,
        },
        permissions::find_permissions_by_id,
    },
    syncing::upgrade,
    translator::translate_permission,
};
use itertools::Itertools;
use rec::model::{
    department::{Department, UpdateDepartment},
    employee::UpdateEmployee,
    name::Name,
    permissions::PermissionName,
    Environment, TableCrud, TableRequest, TableResponse,
};
use tauri::Window;
use uuid::Uuid;

#[tauri::command]
pub async fn search_non_admins(
    app_state: tauri::State<'_, AppState>,
    name: Option<String>,
) -> Result<Vec<Uuid>, String> {
    if let Some(name) = name {
        match find_9_non_admins_by_name(&app_state.pool, &name.trim()).await {
            Ok(ids) => Ok(ids),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_9_non_admins(&app_state.pool).await {
            Ok(ids) => Ok(ids),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[tauri::command]
pub async fn search_admins(app_state: tauri::State<'_, AppState>) -> Result<Vec<Uuid>, String> {
    match find_admins(&app_state.pool).await {
        Ok(ids) => Ok(ids),
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

async fn boss_employee_helper(
    app_state: &AppState,
    window: Window,
    updater_id: Uuid,
    new_boss_id: Uuid,
) -> Result<(), Box<dyn std::error::Error>> {
    let (dep_id, boss_id) =
        find_employee_department_id_and_boss_id(&app_state.pool, new_boss_id).await?;
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
       return Err("null time stamp".into());
   };
    let res = match boss_id {
        Some(_) => {
            main_entry(
                app_state,
                TableRequest::Department(TableCrud::Update(Environment {
                    updater_id,
                    time_stamp,
                    target: UpdateDepartment::ChangeBoss(new_boss_id),
                })),
            )
            .await?
        }
        None => {
            main_entry(
                app_state,
                TableRequest::Department(TableCrud::Update(Environment {
                    updater_id,
                    time_stamp,
                    target: UpdateDepartment::SetBoss(dep_id, new_boss_id),
                })),
            )
            .await?
        }
    };

    match res {
        TableResponse::Done => upgrade(app_state, &window).await,
        TableResponse::Err(err) => return Err(err.into()),
        _ => unreachable!(),
    }

    Ok(())
}

#[tauri::command]
pub async fn boss_employee(
    app_state: tauri::State<'_, AppState>,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    window: Window,
    new_boss_id: Uuid,
) -> Result<(), String> {
    let Some((updater_id, _)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let state = &*app_state;
    match boss_employee_helper(state, window, updater_id, new_boss_id).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

async fn change_password_helper(
    app_state: &AppState,
    updater_id: Uuid,
    employee_id: Uuid,
    old_password: String,
    new_password: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
        return Err("null time stamp".into());
    };
    let employee = find_employee_by_id(&app_state.pool, &employee_id).await?;
    let verified = verify_password(old_password, &employee.password)?;

    if verified {
        let result = main_entry(
            app_state,
            TableRequest::Employee(TableCrud::Update(Environment {
                target: UpdateEmployee::UpdatePassword(employee_id, new_password),
                updater_id,
                time_stamp,
            })),
        )
        .await?;
        match result {
            TableResponse::Done => Ok(()),
            TableResponse::Err(err) => Err(err.into()),
            _ => unreachable!()
        }
    } else {
        Err("كلمة مرور خاطئة".to_string().into())
    }
}

#[tauri::command]
pub async fn change_password(
    app_state: tauri::State<'_, AppState>,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    employee_id: Uuid,
    old_password: String,
    new_password: String,
) -> Result<(), String> {
    let Some((updater_id, _)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let app_state = &*app_state;
    match change_password_helper(
        app_state,
        updater_id,
        employee_id,
        old_password,
        new_password
    ).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
pub async fn find_department(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Department, String> {
    match find_department_by_id(&app_state.pool, id).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn employee_name(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<String, String> {
    match find_employee_name_by_id(&app_state.pool, id).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string()),
    }
}

use strum::IntoEnumIterator;

use crate::tauri_manage::async_commands::verify_password;

#[tauri::command]
pub async fn department_permissions(
    app_state: tauri::State<'_, AppState>,
    department_id: Uuid,
) -> Result<
    (
        Uuid,
        Vec<(String, PermissionName)>,
        Vec<(String, PermissionName)>,
    ),
    String,
> {
    let Ok(Some(boss_id)) = find_department_boss_id_by_id(&app_state.pool, department_id).await else {
        return Err("error getting boss".to_string());
    };
    match find_permissions_by_id(&app_state.pool, boss_id).await {
        Ok(allowed) => {
            let forbidden = PermissionName::iter()
                .filter(|x| !allowed.contains(x))
                .collect_vec();
            let allowed: Vec<(String, PermissionName)> = allowed
                .into_iter()
                .map(|p| (translate_permission(&p), p))
                .collect();
            let forbidden: Vec<(String, PermissionName)> = forbidden
                .into_iter()
                .map(|p| (translate_permission(&p), p))
                .collect();
            Ok((boss_id, allowed, forbidden))
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn employee_permissions_classified(
    app_state: tauri::State<'_, AppState>,
    employee_id: Uuid,
) -> Result<
    (
        Uuid,
        Vec<(String, PermissionName)>,
        Vec<(String, PermissionName)>,
    ),
    String,
> {
    match find_permissions_by_id(&app_state.pool, employee_id).await {
        Ok(allowed) => {
            let forbidden = PermissionName::iter()
                .filter(|x| !allowed.contains(x))
                .collect_vec();
            let allowed: Vec<(String, PermissionName)> = allowed
                .into_iter()
                .map(|p| (translate_permission(&p), p))
                .collect();
            let forbidden: Vec<(String, PermissionName)> = forbidden
                .into_iter()
                .map(|p| (translate_permission(&p), p))
                .collect();
            Ok((employee_id, allowed, forbidden))
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn employee_permissions(
    app_state: tauri::State<'_, AppState>,
    id: Option<Uuid>,
) -> Result<Vec<PermissionName>, String> {
    match id {
        Some(id) => match find_permissions_by_id(&app_state.pool, id).await {
            Ok(permissions) => Ok(permissions),
            Err(err) => Err(err.to_string()),
        },
        None => Ok(vec![]),
    }
}

#[tauri::command]
pub async fn permission_allow(
    app_state: tauri::State<'_, AppState>,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    window: Window,
    employee_id: Uuid,
    permission: PermissionName,
) -> Result<(), String> {
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
       return Err("null time stamp".into());
   };
    let Some((updater_id, _)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let res = match main_entry(
        &app_state,
        TableRequest::Employee(TableCrud::Update(Environment {
            updater_id,
            time_stamp,
            target: UpdateEmployee::AllowPermission(employee_id, permission),
        })),
    )
    .await
    {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };
    match res {
        TableResponse::Done => upgrade(&app_state, &window).await,
        TableResponse::Err(err) => return Err(err.into()),
        _ => unreachable!(),
    }
    Ok(())
}

#[tauri::command]
pub async fn permission_forbid(
    app_state: tauri::State<'_, AppState>,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    window: Window,
    employee_id: Uuid,
    permission: PermissionName,
) -> Result<(), String> {
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
       return Err("null time stamp".into());
   };
    let Some((updater_id, _)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let res = match main_entry(
        &app_state,
        TableRequest::Employee(TableCrud::Update(Environment {
            updater_id,
            time_stamp,
            target: UpdateEmployee::ForbidPermission(employee_id, permission),
        })),
    )
    .await
    {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    match res {
        TableResponse::Done => upgrade(&app_state, &window).await,
        TableResponse::Err(err) => return Err(err.into()),
        _ => unreachable!(),
    }
    Ok(())
}

#[tauri::command]
pub async fn admin_employee(
    app_state: tauri::State<'_, AppState>,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    window: Window,
    employee_id: Uuid,
) -> Result<(), String> {
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
       return Err("null time stamp".into());
   };
    let Some((updater_id, _)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let res = match main_entry(
        &app_state,
        TableRequest::Employee(TableCrud::Update(Environment {
            updater_id,
            time_stamp,
            target: UpdateEmployee::Up(employee_id),
        })),
    )
    .await
    {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    match res {
        TableResponse::Done => upgrade(&app_state, &window).await,
        TableResponse::Err(err) => return Err(err.into()),
        _ => unreachable!(),
    }
    Ok(())
}

#[tauri::command]
pub async fn unadmin_employee(
    app_state: tauri::State<'_, AppState>,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    window: Window,
    employee_id: Uuid,
) -> Result<(), String> {
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
       return Err("null time stamp".into());
   };
    let Some((updater_id, _)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let res = match main_entry(
        &app_state,
        TableRequest::Employee(TableCrud::Update(Environment {
            updater_id,
            time_stamp,
            target: UpdateEmployee::Down(employee_id),
        })),
    )
    .await
    {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    match res {
        TableResponse::Done => upgrade(&app_state, &window).await,
        TableResponse::Err(err) => return Err(err.into()),
        _ => unreachable!(),
    }
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
