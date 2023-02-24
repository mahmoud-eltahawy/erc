use std::error::Error;

use uuid::Uuid;

use crate::config::AppState;

pub async fn save_shift(app_state : &AppState,department_id : &String) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let department_id = Uuid::parse_str(department_id)?;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/shift/save"))
    .json(&department_id)
    .send()
    .await?;

    Ok(())
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
