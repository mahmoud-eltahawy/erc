use std::error::Error;

use rec::model::machine::Machine;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_machine(app_state : &AppState,machine : &Machine<Uuid>) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .post(format!("{origin}/machine/"))
    .json(machine)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn update_machine(app_state : &AppState,machine : &Machine<Uuid>) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .put(format!("{origin}/machine/"))
    .json(machine)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn delete_machine(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .delete(format!("{origin}/machine/{id}"))
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}
