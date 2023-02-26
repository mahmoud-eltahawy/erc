use std::error::Error;

use reqwest::StatusCode;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_shift(app_state : &AppState,id : &String) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let id = Uuid::parse_str(id)?;
  let req = reqwest::Client::new()
    .post(format!("{origin}/shift/{id}"))
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn delete_shift(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .delete(format!("{origin}/shift/{id}"))
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}
