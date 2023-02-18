use std::error::Error;

use errc::{
  config::AppState,
  api::for_selection::{
    all_machines,
    all_spare_parts
  }, syncing::upgrade, test::insert_employees, memory::employee::find_all_employees
};
use rec::{timer::{get_relative_now, get_current_order}, model::name::Name};
use uuid::Uuid;

use super::models::TauriState;
use rusqlite::Connection;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

async fn try_connect() -> Result<Pool<Sqlite>,Box<dyn Error>>{
  match SqlitePoolOptions::new()
    .max_connections(10)
    .connect("sqlite:memory.db")
    .await {
    Ok(pool) => Ok(pool),
    Err(err) => Err(err.into())
  }
}

async fn get_pool() -> Result<Pool<Sqlite>,Box<dyn Error>>{
  match try_connect().await {
    Ok(p) => Ok(p),
    Err(_) => {
      let con = Connection::open("memory.db")?;
      con.close().unwrap();
      let p = try_connect().await?;

      sqlx::migrate!("db/migrations")
        .run(&p)
        .await?;

      Ok(p)
    }
  }
}

async fn test(app_state : &AppState) -> Result<(),Box<dyn Error>>{
  insert_employees(app_state).await?;
  Ok(())
}

pub async fn create_tauri_state() -> Result<TauriState,Box<dyn std::error::Error>>{
  let pool = get_pool().await?;

  let app_state = AppState::new(pool);
  let relative_now = get_relative_now();
  let order = get_current_order(relative_now);

  test(&app_state).await?;

  let employees = match find_all_employees(&app_state.pool).await {
    Ok(e) => e.into_iter().map(|emp| Name{id : Some(Uuid::parse_str(&emp.id).unwrap()),
                      name : format!("{} {} {}",emp.first_name,emp.middle_name,emp.last_name)}).collect(),
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

  upgrade(&app_state).await?;
  Ok(TauriState{
    app_state,
    relative_now,
    order,
    employees,
    machines,
    spare_parts
  })
}
