use errc::{
  memory::employee::{
    find_4_non_admins_by_name,
    find_4_non_admins,
    find_admins
  },
  config::AppState,
  api::employee::{
    up_employee,
    down_employee
  },
  syncing::upgrade
};
use rec::model::name::Name;
use tauri::Window;
use uuid::Uuid;

#[tauri::command]
pub async fn search_non_admins(app_state : tauri::State<'_,AppState>,name : Option<String>) -> Result<Vec<Name>,String> {
  if let Some(name) = name {
    match find_4_non_admins_by_name(&app_state.pool,&name.trim()).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
  } else {
    match find_4_non_admins(&app_state.pool).await {
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
