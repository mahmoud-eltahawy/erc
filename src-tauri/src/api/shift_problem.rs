use std::error::Error;

use rec::model::shift_problem::ShiftProblem;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_shift_problem(app_state : &AppState,problem : &ShiftProblem) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/shift-problem/save"))
      .json(problem)
      .send()
      .await?;

  Ok(())
}

pub async fn update_shift_problem(app_state : &AppState,problem : &ShiftProblem) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/shift-problem/update"))
      .json(problem)
      .send()
      .await?;

  Ok(())
}

pub async fn delete_shift_problem(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/shift-problem/delete"))
      .json(id)
      .send()
      .await?;

  Ok(())
}
