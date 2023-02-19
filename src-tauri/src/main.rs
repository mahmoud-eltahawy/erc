#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod tauri_manage;
use std::error::Error;

use tauri_manage::app;

use dotenv::dotenv;


#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
  dotenv().ok();
  let app = app().await?;
  let _ = app.run(tauri::generate_context!())?;
  Ok(())
}
