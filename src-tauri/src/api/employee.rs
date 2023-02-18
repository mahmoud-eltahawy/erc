use rec::model::employee::Employee;
use uuid::Uuid;
use std::error::Error;

use crate::config::AppState;

pub async fn fetch_employee_by_id(app_state : &AppState,id : Uuid) -> Result<Employee,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/emp/emp"))
      .json(&id)
      .send()
      .await?
      .json::<Employee>()
      .await?;

  Ok(result)
}

pub async fn save_employee(app_state : &AppState,employee : &Employee) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/emp/save"))
      .json(employee)
      .send()
      .await?;

  Ok(())
}

pub async fn update_employee(app_state : &AppState,employee : &Employee) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/emp/update"))
      .json(employee)
      .send()
      .await?;

  Ok(())
}

pub async fn delete_employee(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/emp/delete"))
      .json(id)
      .send()
      .await?;

  Ok(())
}