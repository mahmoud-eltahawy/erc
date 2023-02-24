use std::error::Error;

use reqwest::StatusCode;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_problem_to_shift_problem(app_state : &AppState
                              ,pid : &Uuid,spid : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .get(format!("{}/api/sp/problem/{}/{}",origin,pid,spid))
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn delete_problem_from_shift_problem(app_state : &AppState,
                                pid : &Uuid,spid : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .delete(format!("{}/api/sp/problem/{}/{}",origin,pid,spid))
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn save_spare_part_to_shift_problem(app_state : &AppState,
                              pid : &Uuid,spid : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .get(format!("{}/api/sp/part/{}/{}",origin,pid,spid))
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn delete_spare_part_from_shift_problem(app_state : &AppState,
                                  pid : &Uuid,spid : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .delete(format!("{}/api/sp/part/{}/{}",origin,pid,spid))
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}
