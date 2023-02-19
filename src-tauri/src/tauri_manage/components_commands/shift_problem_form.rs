use chrono::NaiveTime;
use errc::{
    config::AppState,
    memory::{
        problem::find_problems_names_by_department_id,
        employee::find_all_employees_names,
        machine::find_all_machines_names,
        spare_part::find_all_spare_parts_names
    }
};
use rec::{model::name::Name, timer::{get_shift_borders, get_relative_now, get_current_order}};
use uuid::Uuid;

#[tauri::command]
pub async fn problems_selection(app_state : tauri::State<'_,AppState>,
                            department_id : Uuid) -> Result<Vec<Name>,String> {
  match find_problems_names_by_department_id(&app_state.pool,department_id.to_string()).await {
    Ok(p) => Ok(p),
    Err(err)=> Err(err.to_string())
  }
}

#[tauri::command]
pub async fn employees_selection(app_state : tauri::State<'_,AppState>) -> Result<Vec<Name>,String> {
  match find_all_employees_names(&app_state.pool).await {
    Ok(e) => Ok(e),
    Err(err)=> Err(err.to_string())
  }
}

#[tauri::command]
pub async fn machines_selection(app_state : tauri::State<'_,AppState>) -> Result<Vec<Name>,String> {
  match find_all_machines_names(&app_state.pool).await {
    Ok(m) => Ok(m),
    Err(err)=> Err(err.to_string())
  }
}

#[tauri::command]
pub async fn spare_parts_selection(app_state : tauri::State<'_,AppState>) -> Result<Vec<Name>,String> {
   match find_all_spare_parts_names(&app_state.pool).await {
    Ok(s) => Ok(s),
    Err(err)=> Err(err.to_string())
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
