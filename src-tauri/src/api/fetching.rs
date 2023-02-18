use crate::config::AppState;

use rec::model::shift_problem::{MinimamlShiftProblem, WriterAndShiftIds};

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
