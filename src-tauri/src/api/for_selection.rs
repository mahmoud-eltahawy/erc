use std::error::Error;
use crate::config::AppState;
use rec::model::{
  machine::Machine,
  spare_part::SparePart,
  name::Name
};

pub async fn all_machines(app_state : &AppState) -> Result<Vec<Name>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.get(format!("{origin}/api/machine/all"))
      .send()
      .await?
      .json::<Vec<Machine>>()
      .await?;

  let result : Vec<Name> = result
    .into_iter().map(|p| Name{id : p.id.to_string(),name : p.name}).collect();

  Ok(result)
}

pub async fn all_spare_parts(app_state : &AppState) -> Result<Vec<Name>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.get(format!("{origin}/api/spare-part/all"))
      .send()
      .await?
      .json::<Vec<SparePart>>()
      .await?;

  let result : Vec<Name> = result
    .into_iter().map(|s| Name{id: s.id.to_string(), name : s.name}).collect();

  Ok(result)
}
