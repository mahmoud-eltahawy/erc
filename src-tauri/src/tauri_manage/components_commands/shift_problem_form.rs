use chrono::NaiveTime;
use errc::{
    config::AppState,
    memory::{
        problem::{find_department_problems_by_name, find_department_4_problems},
        employee::{find_employees_by_name, find_4_employees},
        machine::{find_machines_by_name, find_4_machines},
        spare_part::{find_spare_parts_by_name, find_4_spare_parts}
    }
};
use rec::{model::name::Name, timer::{get_shift_borders, get_relative_now, get_current_order}};
use uuid::Uuid;

#[tauri::command]
pub async fn problems_selection(app_state : tauri::State<'_,AppState>,
                            department_id : Uuid,name : Option<String>,canceled : Vec<String>) -> Result<Vec<Name>,String> {
  if let Some(name) = &name {
    match find_department_problems_by_name(&app_state.pool,department_id.to_string(),name,canceled).await {
      Ok(p) => Ok(p),
      Err(err) => Err(err.to_string())
    }
  } else {
    match find_department_4_problems(&app_state.pool,department_id.to_string(),canceled).await {
      Ok(p) => Ok(p),
      Err(err)=> Err(err.to_string())
    }
  }
}

#[tauri::command]
pub async fn employees_selection(app_state : tauri::State<'_,AppState>,
                            name : Option<String>,canceled : Vec<String>) -> Result<Vec<Name>,String> {
  if let Some(name) = name {
    match find_employees_by_name(&app_state.pool,&name,canceled).await {
      Ok(e) => Ok(e),
      Err(err)=> Err(err.to_string())
    }
  } else {
    match find_4_employees(&app_state.pool,canceled).await {
      Ok(e) => Ok(e),
      Err(err)=> Err(err.to_string())
    }
  }
}

#[tauri::command]
pub async fn machines_selection(app_state : tauri::State<'_,AppState>,
                            name : Option<String>,canceled : Vec<String>) -> Result<Vec<Name>,String> {
  if let Some(name) = name {
    match find_machines_by_name(&app_state.pool,&name,canceled).await {
      Ok(e) => Ok(e),
      Err(err)=> Err(err.to_string())
    }
  } else {
    match find_4_machines(&app_state.pool,canceled).await {
      Ok(e) => Ok(e),
      Err(err)=> Err(err.to_string())
    }
  }
}

#[tauri::command]
pub async fn spare_parts_selection(app_state : tauri::State<'_,AppState>,
                            name : Option<String>,canceled : Vec<String>) -> Result<Vec<Name>,String> {
  if let Some(name) = name {
    match find_spare_parts_by_name(&app_state.pool,&name,canceled).await {
      Ok(e) => Ok(e),
      Err(err)=> Err(err.to_string())
    }
  } else {
    match find_4_spare_parts(&app_state.pool,canceled).await {
      Ok(e) => Ok(e),
      Err(err)=> Err(err.to_string())
    }
  }
}

#[tauri::command]
pub fn current_shift_borders() -> Result<(NaiveTime, NaiveTime), String> {
  let relative_now = get_relative_now();
  let order = get_current_order(relative_now);
  match get_shift_borders(order) {
      Some(v) => Ok(v),
      None    => Err("shift borders is null".to_string())
  }
}
