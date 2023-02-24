use rec::model::employee::Employee;
use reqwest::StatusCode;
use uuid::Uuid;
use std::error::Error;

use crate::config::AppState;

pub async fn save_employee(app_state : &AppState,employee : &Employee) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .post(format!("{}/api/emp/",origin))
    .json(employee)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn update_employee(app_state : &AppState,employee : &Employee) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .put(format!("{}/api/emp/",origin))
    .json(employee)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn delete_employee(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .post(format!("{}/api/emp/{}",origin,id))
    .json(id)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}
