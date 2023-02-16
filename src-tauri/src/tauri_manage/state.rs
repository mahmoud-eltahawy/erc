use errc::{config::AppState, api::for_selection::{all_employees, all_machines, all_spare_parts}};
use rec::timer::{get_relative_now, get_current_order};

use super::models::TauriState;
use rusqlite::Connection;
use sqlx::SqlitePool;

pub async fn create_tauri_state() -> Result<TauriState,Box<dyn std::error::Error>>{
  let pool = match SqlitePool::connect("memory.db").await{
    Ok(p) => p,
    Err(_) => {
      Connection::open("memory.db")?;
      let p = SqlitePool::connect("memory.db").await?;
      p
    }
  };

  let app_state = AppState::new(pool);
  let relative_now = get_relative_now();
  let order = get_current_order(relative_now);

  let employees = match all_employees(&app_state).await {
    Ok(e) => e,
    Err(err)=> return Err(err.into())
  };

  let machines = match all_machines(&app_state).await {
    Ok(m) => m,
    Err(err)=> return Err(err.into())
  };

  let spare_parts = match all_spare_parts(&app_state).await {
    Ok(s) => s,
    Err(err)=> return Err(err.into())
  };

  Ok(TauriState{
    app_state,
    relative_now,
    order,
    employees,
    machines,
    spare_parts
  })
}
