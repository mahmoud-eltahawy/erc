use errc::{
  memory::{
    employee::{
      find_9_non_admins_by_name,
      find_9_non_admins,
      find_admins,
      find_employee_name_by_id,
      find_employees_by_department_id_except_boss
  },
  department::{
    find_all_departments,
    find_department_by_id
  }},
  config::AppState,
  api::{employee::{
    up_employee,
    down_employee
  }, department::sets_department_boss},
  syncing::upgrade
};
use rec::model::{name::Name, department::ClientDepartment};
use tauri::Window;
use uuid::Uuid;

#[tauri::command]
pub async fn search_non_admins(app_state : tauri::State<'_,AppState>,name : Option<String>) -> Result<Vec<Name>,String> {
  if let Some(name) = name {
    match find_9_non_admins_by_name(&app_state.pool,&name.trim()).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
  } else {
    match find_9_non_admins(&app_state.pool).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
  }
}

#[tauri::command]
pub async fn search_admins(app_state : tauri::State<'_,AppState>) -> Result<Vec<Name>,String> {
  match find_admins(&app_state.pool).await {
    Ok(days) => Ok(days),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
pub async fn list_departments(app_state : tauri::State<'_,AppState>) -> Result<Vec<Name>,String> {
  match find_all_departments(&app_state.pool).await {
    Ok(days) => Ok(days),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
pub async fn department_employees(app_state : tauri::State<'_,AppState>,id : Uuid) -> Result<Vec<Name>,String> {
  match find_employees_by_department_id_except_boss(&app_state.pool,id.to_string()).await {
    Ok(days) => Ok(days),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
pub async fn boss_employee(app_state : tauri::State<'_,AppState>,id : Uuid,window : Window) -> Result<(),String> {
  let Ok(_) = sets_department_boss(&app_state,&id).await else {
    return Err("".to_string())
  };

  let Ok(()) = upgrade(&app_state, Some(&window)).await else {
    return Err("".to_string());
  };

  Ok(())
}

#[tauri::command]
pub async fn find_department(app_state : tauri::State<'_,AppState>,id : Uuid) -> Result<ClientDepartment,String> {
  match find_department_by_id(&app_state.pool,id.to_string()).await {
    Ok(days) => Ok(days),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
pub async fn employee_name(app_state : tauri::State<'_,AppState>,id : Uuid) -> Result<String,String> {
  match find_employee_name_by_id(&app_state.pool,id.to_string()).await {
    Ok(days) => Ok(days),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
pub async fn admin_employee(app_state : tauri::State<'_,AppState>,window : Window,id : Uuid) -> Result<(),String> {
  let Ok(()) = up_employee(&app_state,&id).await else {
    return Err("".to_string());
  };
  let Ok(()) = upgrade(&app_state, Some(&window)).await else {
    return Err("".to_string());
  };
  Ok(())
}

#[tauri::command]
pub async fn unadmin_employee(app_state : tauri::State<'_,AppState>,window : Window,id : Uuid) -> Result<(),String> {
  let Ok(()) = down_employee(&app_state,&id).await else {
    return Err("".to_string());
  };
  let Ok(()) = upgrade(&app_state, Some(&window)).await else {
    return Err("".to_string());
  };
  Ok(())
}
