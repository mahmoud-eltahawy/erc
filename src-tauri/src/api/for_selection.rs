use std::error::Error;
use crate::config::AppState;
use rec::model::{
  employee::Employee,
  problem::Probelm,
  machine::Machine,
  spare_part::SparePart,
  name::Name
};
use uuid::Uuid;

pub async fn all_employees(app_state : &AppState) -> Result<Vec<Name>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.get(format!("{origin}/api/emp/all"))
      .send()
      .await?
      .json::<Vec<Employee>>()
      .await?;

  let result : Vec<Name> = result
    .iter().map(|e| Name::build_employee(e)).collect();

  Ok(result)
}

pub async fn all_problems(app_state : &AppState,department_id: Uuid) -> Result<Vec<Name>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.get(format!("{origin}/api/problem/all"))
      .json(&department_id)
      .send()
      .await?
      .json::<Vec<Probelm>>()
      .await?;

  let result : Vec<Name> = result
    .iter().map(|p| Name::build_problem(p)).collect();

  Ok(result)
}

pub async fn all_machines(app_state : &AppState) -> Result<Vec<Name>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.get(format!("{origin}/api/machine/all"))
      .send()
      .await?
      .json::<Vec<Machine>>()
      .await?;

  let result : Vec<Name> = result
    .iter().map(|p| Name::build_machine(p)).collect();

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
    .iter().map(|s| Name::build_spare_part(s)).collect();

  Ok(result)
}
