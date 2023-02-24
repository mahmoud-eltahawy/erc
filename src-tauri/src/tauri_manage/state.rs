use std::error::Error;

use errc::{
  config::AppState,
  syncing::upgrade,
  test::insert_employees,
};

use super::models::TauriState;
use rusqlite::Connection;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

async fn try_connect(path: &String) -> Result<Pool<Sqlite>,Box<dyn Error>>{
  match SqlitePoolOptions::new()
    .max_connections(10)
    .connect(&path)
    .await {
    Ok(pool) => Ok(pool),
    Err(err) => Err(err.into())
  }
}

async fn get_pool() -> Result<Pool<Sqlite>,Box<dyn Error>>{
  let db_path = std::env::var("DATABASE_URL").expect("ivalid database url");
  match try_connect(&db_path).await {
    Ok(p) => Ok(p),
    Err(_) => {
      let db_path_only = db_path.split(":").collect::<Vec<&str>>();
      let db_path_only = db_path_only.get(1).expect("failed to execute db path");
      let con = Connection::open(db_path_only)?;
      con.close().unwrap();
      let p = try_connect(&db_path).await?;

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

pub async fn create_tauri_state() -> Result<TauriState,Box<dyn Error>>{
  let pool = get_pool().await?;

  let app_state = AppState::new(pool);

  test(&app_state).await?;

  upgrade(&app_state).await?;
  Ok(TauriState{
    app_state,
  })
}
