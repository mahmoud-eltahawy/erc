use std::error::Error;
use rec::model::department::Department;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_department(app_state : &AppState,dep : &Department) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/dep/save"))
      .json(dep)
      .send()
      .await?;

  Ok(())
}

pub async fn update_department(app_state : &AppState,dep : &Department) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/dep/update"))
      .json(dep)
      .send()
      .await?;

  Ok(())
}

pub async fn delete_department(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/dep/delete"))
      .json(id)
      .send()
      .await?;

  Ok(())
}
