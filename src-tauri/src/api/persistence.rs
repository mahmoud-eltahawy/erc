use uuid::Uuid;

use crate::{model::{ShiftProblem, Probelm}, config::AppState};

pub async fn save_problem_detail(app_state : &AppState,shift_problem :&ShiftProblem) -> Result<Uuid,Box<dyn std::error::Error>> {
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

pub async fn save_problem(app_state : &AppState,problem :&Probelm) -> Result<bool,Box<dyn std::error::Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/problem/save"))
      .json(problem)
      .send()
      .await?
      .json::<bool>()
      .await?;

  Ok(result)
}
