use std::error::Error;

use rec::model::note::{DbNote, Note};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_note_to_problem(app_state : &AppState,note : &DbNote) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .post(format!("{}/api/note/problem",origin))
    .json(note)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn save_note_to_shift(app_state : &AppState,note : &DbNote) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .post(format!("{}/api/note/shift",origin))
    .json(note)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn update_note(app_state : &AppState,note : &Note) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .put(format!("{origin}/api/note/"))
    .json(note)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn delete_note(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .delete(format!("{}/api/note/{}",origin,id))
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}
