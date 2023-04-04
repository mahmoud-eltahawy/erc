use std::error::Error;

use rec::model::spare_part::SparePart;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_spare_part(app_state : &AppState,spare_part : &SparePart<Uuid>) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .post(format!("{origin}/part/"))
    .json(spare_part)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn update_spare_part(app_state : &AppState,spare_part : &SparePart<Uuid>) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .put(format!("{origin}/part/"))
    .json(spare_part)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn delete_spare_part(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .delete(format!("{origin}/part/{id}"))
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}
