use std::error::Error;

use rec::model::note::{DbNote, Note};
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_note_to_problem(app_state : &AppState,note : &DbNote) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/note/p-save"))
      .json(note)
      .send()
      .await?;

  Ok(())
}

pub async fn save_note_to_shift(app_state : &AppState,note : &DbNote) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/note/s-save"))
      .json(note)
      .send()
      .await?;

  Ok(())
}

pub async fn update_note(app_state : &AppState,note : &Note) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/note/update"))
      .json(note)
      .send()
      .await?;

  Ok(())
}

pub async fn delete_note(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/note/delete"))
      .json(id)
      .send()
      .await?;

  Ok(())
}
