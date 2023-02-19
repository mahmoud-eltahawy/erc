use std::error::Error;

use rec::model::relations::{ShiftProblemProblem, ShiftProblemSparePart};

use crate::config::AppState;

pub async fn save_problem_to_shift_problem(app_state : &AppState
                              ,sp : &ShiftProblemProblem) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/relation/sp/p-save"))
      .json(sp)
      .send()
      .await?;

  Ok(())
}

pub async fn delete_problem_from_shift_problem(app_state : &AppState,
                                sp : &ShiftProblemProblem) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/relation/sp/p-delete"))
      .json(sp)
      .send()
      .await?;

  Ok(())
}

pub async fn save_spare_part_to_shift_problem(app_state : &AppState,
                              sp : &ShiftProblemSparePart) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/relation/sp/s-save"))
      .json(sp)
      .send()
      .await?;

  Ok(())
}

pub async fn delete_spare_part_from_shift_problem(app_state : &AppState,
                                  sp : &ShiftProblemSparePart) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/relation/sp/s-delete"))
      .json(sp)
      .send()
      .await?;

  Ok(())
}
