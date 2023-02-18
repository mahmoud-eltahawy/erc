use std::error::Error;

use rec::model::problem::Probelm;
use uuid::Uuid;

use crate::config::AppState;

pub async fn fetch_problem_by_id(app_state : &AppState,id : Uuid) -> Result<Probelm,Box<dyn Error>> {
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

pub async fn save_problem(app_state : &AppState,shift_problem :&Probelm) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/problem/save"))
      .json(shift_problem)
      .send()
      .await?;

  Ok(())
}

pub async fn update_problem(app_state : &AppState,problem : &Probelm) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/problem/update"))
      .json(problem)
      .send()
      .await?;

  Ok(())
}

pub async fn delete_problem(app_state : &AppState, id : &Uuid) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  client.post(format!("{origin}/api/problem/delete"))
      .json(id)
      .send()
      .await?;

  Ok(())
}
