use std::error::Error;

use rec::model::spare_part::SparePart;
use uuid::Uuid;

use crate::config::AppState;

pub async fn fetch_spare_part_by_id(app_state : &AppState,id : Uuid) -> Result<SparePart,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/spare-part/part"))
      .json(&id)
      .send()
      .await?
      .json::<SparePart>()
      .await?;

  Ok(result)
}

pub async fn save_spare_part(app_state : &AppState,spare_part : &SparePart) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/spare-part/save"))
      .json(spare_part)
      .send()
      .await?;

  Ok(())
}

pub async fn update_spare_part(app_state : &AppState,spare_part : &SparePart) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/spare-part/update"))
      .json(spare_part)
      .send()
      .await?;

  Ok(())
}

pub async fn delete_spare_part(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/spare-part/delete"))
      .json(id)
      .send()
      .await?;

  Ok(())
}
