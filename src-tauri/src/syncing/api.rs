use std::error::Error;

use rec::{
  crud_sync::CudVersion,
  model::{
    employee::Employee,
    problem::Problem,
    shift::{
      Shift,
      DepartmentShift
    },
    spare_part::SparePart,
    department::Department,
    machine::Machine,
    note::DbNote,
    shift_problem::ShiftProblem,
    permissions::Permissions
  }
};
use uuid::Uuid;

use crate::config::AppState;

pub async fn updates(app_state : &AppState,
        version : u64) -> Result<Vec<CudVersion>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/sync/{version}"))
    .send()
    .await?
    .json::<Vec<CudVersion>>()
    .await?;

  Ok(result)
}

pub async fn shift(app_state : &AppState,id : Uuid) -> Result<Shift,Box<dyn Error>> {
  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/shift/{id}"))
    .send()
    .await?
    .json::<Shift>()
    .await?;

  Ok(result)
}

pub async fn shift_department(app_state : &AppState,id : Uuid) -> Result<DepartmentShift,Box<dyn Error>> {
  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/shift/dep/{id}"))
    .send()
    .await?
    .json::<DepartmentShift>()
    .await?;

  Ok(result)
}

pub async fn employee(app_state : &AppState,id : Uuid) -> Result<Employee<Uuid>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/emp/{id}"))
    .send()
    .await?
    .json::<Employee<Uuid>>()
    .await?;

  Ok(result)
}

pub async fn permissions(app_state : &AppState,id : Uuid) -> Result<Permissions<Uuid>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/per/{id}"))
    .send()
    .await?
    .json::<Permissions<Uuid>>()
    .await?;

  Ok(result)
}

pub async fn problem(app_state : &AppState,id : Uuid) -> Result<Problem<Uuid>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/problem/{id}"))
    .send()
    .await?
    .json::<Problem<Uuid>>()
    .await?;

  Ok(result)
}

pub async fn spare_part(app_state : &AppState,id : Uuid) -> Result<SparePart<Uuid>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/part/{id}"))
    .send()
    .await?
    .json::<SparePart<Uuid>>()
    .await?;

  Ok(result)
}

pub async fn department(app_state : &AppState,id : Uuid) -> Result<Department<Uuid>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/dep/{id}"))
    .send()
    .await?
    .json::<Department<Uuid>>()
    .await?;

  Ok(result)
}

pub async fn machine(app_state : &AppState,id : Uuid) -> Result<Machine<Uuid>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/machine/{id}"))
    .send()
    .await?
    .json::<Machine<Uuid>>()
    .await?;

  Ok(result)
}

pub async fn note(app_state : &AppState,id : Uuid) -> Result<DbNote<Uuid>,Box<dyn Error>> {
  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/note/{id}"))
    .send()
    .await?
    .json::<DbNote<Uuid>>()
    .await?;

  Ok(result)
}

pub async fn shift_problem(app_state : &AppState,id : Uuid) -> Result<ShiftProblem,Box<dyn Error>> {
  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/sp/{id}"))
    .send()
    .await?
    .json::<ShiftProblem>()
    .await?;

  Ok(result)
}
