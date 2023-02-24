use std::error::Error;
use rec::model::department::Department;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_department(app_state : &AppState,dep : &Department) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .post(format!("{}/api/dep/",origin))
    .json(dep)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn update_department(app_state : &AppState,dep : &Department) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .put(format!("{}/api/dep/",origin))
    .json(dep)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn delete_department(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .delete(format!("{}/api/dep/{}",origin,id))
    .json(id)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}
