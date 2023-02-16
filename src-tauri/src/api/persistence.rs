use uuid::Uuid;

use crate::config::AppState;
use rec::model::{
  problem::Probelm,
  shift_problem::MinimamlShiftProblem, employee::Employee
};

pub async fn save_problem_detail(app_state : &AppState,shift_problem :&MinimamlShiftProblem) -> Result<Uuid,Box<dyn std::error::Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/sp/save"))
      .json(shift_problem)
      .send()
      .await?
      .json::<Option<Uuid>>()
      .await?;

  match result {
    Some(id) => Ok(id),
    None     => Err("not found".into())
  }
}

pub async fn save_problem(app_state : &AppState,shift_problem :&Probelm) -> Result<bool,Box<dyn std::error::Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/problem/save"))
      .json(shift_problem)
      .send()
      .await?
      .json::<bool>()
      .await?;

  Ok(result)
}

pub async fn save_employee(app_state : &AppState,employee : &Employee) -> Result<(),Box<dyn std::error::Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/emp/save"))
      .json(employee)
      .send()
      .await?;

  Ok(())
}
