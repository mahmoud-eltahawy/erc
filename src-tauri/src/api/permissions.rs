use std::error::Error;
use rec::model::permissions::Permissions;
use reqwest::StatusCode;

use crate::config::AppState;

pub async fn save_permissions(app_state : &AppState,permissions : &Permissions) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .post(format!("{origin}/per/"))
    .json(permissions)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn update_department(app_state : &AppState,permissions : &Permissions) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .put(format!("{origin}/per/"))
    .json(permissions)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}
