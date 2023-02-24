use std::error::Error;

use rec::model::machine::Machine;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_machine(app_state : &AppState,machine : &Machine) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/machine/save"))
      .json(machine)
      .send()
      .await?;

  Ok(())
}

pub async fn update_machine(app_state : &AppState,machine : &Machine) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/machine/update"))
      .json(machine)
      .send()
      .await?;

  Ok(())
}

pub async fn delete_machine(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/machine/delete"))
      .json(id)
      .send()
      .await?;

  Ok(())
}