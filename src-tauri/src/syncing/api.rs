use std::error::Error;

use rec::{crud_sync::CudVersion, model::{employee::Employee, problem::Probelm, shift::{Shift, DepartmentShift}, spare_part::SparePart, department::Department, machine::Machine, note::DbNote, shift_problem::ShiftProblem}};
use uuid::Uuid;

use crate::config::AppState;

pub async fn updates(app_state : &AppState,
        current_version : u64) -> Result<Vec<CudVersion>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/sync/update"))
      .json(&current_version)
      .send()
      .await?
      .json::<Vec<CudVersion>>()
      .await?;

  Ok(result)
}

pub async fn shift(app_state : &AppState,id : Uuid) -> Result<Shift,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/shift/shift"))
      .json(&id)
      .send()
      .await?
      .json::<Shift>()
      .await?;

  Ok(result)
}

pub async fn shift_department(app_state : &AppState,id : Uuid) -> Result<DepartmentShift,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/shift/dep-shift"))
      .json(&id)
      .send()
      .await?
      .json::<DepartmentShift>()
      .await?;

  Ok(result)
}

pub async fn employee(app_state : &AppState,id : Uuid) -> Result<Employee,Box<dyn Error>> {
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

pub async fn problem(app_state : &AppState,id : Uuid) -> Result<Probelm,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/problem/problem"))
      .json(&id)
      .send()
      .await?
      .json::<Probelm>()
      .await?;

  Ok(result)
}

pub async fn spare_part(app_state : &AppState,id : Uuid) -> Result<SparePart,Box<dyn Error>> {
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

pub async fn department(app_state : &AppState,id : Uuid) -> Result<Department,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/dep/dep"))
      .json(&id)
      .send()
      .await?
      .json::<Department>()
      .await?;

  Ok(result)
}

pub async fn machine(app_state : &AppState,id : Uuid) -> Result<Machine,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/machine/machine"))
      .json(&id)
      .send()
      .await?
      .json::<Machine>()
      .await?;

  Ok(result)
}

pub async fn note(app_state : &AppState,id : Uuid) -> Result<DbNote,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/note/note"))
      .json(&id)
      .send()
      .await?
      .json::<DbNote>()
      .await?;

  Ok(result)
}

pub async fn shift_problem(app_state : &AppState,id : Uuid) -> Result<ShiftProblem,Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/shift-problem/shift-problem"))
      .json(&id)
      .send()
      .await?
      .json::<ShiftProblem>()
      .await?;

  Ok(result)
}
