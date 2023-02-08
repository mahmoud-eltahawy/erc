use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::config::AppState;

use rec::model::{
  employee::Employee,
  problem::Probelm,
  machine::Machine,
  spare_part::SparePart,
  shift_problem::MinimamlShiftProblem
};

#[derive(Serialize,Deserialize)]
pub struct WriterAndShiftIds{
  pub writer_id     : Uuid,
  pub shift_id      : Uuid,
}

pub async fn fetch_current_problem_detail(app_state : &AppState,
      writer_shift_ids : WriterAndShiftIds) -> Result<Vec<MinimamlShiftProblem>,Box<dyn std::error::Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.get(format!("{origin}/api/sp/cproblems"))
      .json(&writer_shift_ids)
      .send()
      .await?
      .json::<Option<Vec<MinimamlShiftProblem>>>()
      .await?;

  match result {
    Some(problems) => Ok(problems),
    None     => Err("not found".into())
  }
}

pub async fn fetch_employee_by_id(app_state : &AppState,id : Uuid) -> Result<Employee,Box<dyn std::error::Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/emp/emp"))
      .json(&id)
      .send()
      .await?
      .json::<Option<Employee>>()
      .await?;

  match result {
    Some(emp) => Ok(emp),
    None     => Err("not found".into())
  }
}

pub async fn fetch_problem_by_id(app_state : &AppState,id : Uuid) -> Result<Probelm,Box<dyn std::error::Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/problem/problem"))
      .json(&id)
      .send()
      .await?
      .json::<Option<Probelm>>()
      .await?;

  match result {
    Some(p) => Ok(p),
    None     => Err("not found".into())
  }
}

pub async fn fetch_spare_part_by_id(app_state : &AppState,id : Uuid) -> Result<SparePart,Box<dyn std::error::Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/spare-part/part"))
      .json(&id)
      .send()
      .await?
      .json::<Option<SparePart>>()
      .await?;

  match result {
    Some(emp) => Ok(emp),
    None     => Err("not found".into())
  }
}

pub async fn fetch_machine_by_id(app_state : &AppState,id : Uuid) -> Result<Machine,Box<dyn std::error::Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/machine/machine"))
      .json(&id)
      .send()
      .await?
      .json::<Option<Machine>>()
      .await?;

  match result {
    Some(mac) => Ok(mac),
    None     => Err("not found".into())
  }
}
