use std::error::Error;

use rec::model::shift::Shift;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_shift_or(app_state : &AppState) -> Result<Shift,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/shift/save-or"))
    .send()
    .await?
    .json::<Option<Shift>>()
    .await?;

  match result {
      Some(shift) => Ok(shift),
      None        => Err("shift create error".to_owned().into())
  }
}

pub async fn delete_shift(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/shift/delete"))
      .json(id)
      .send()
      .await?;

  Ok(())
}
