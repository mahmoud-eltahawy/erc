use uuid::Uuid;

use crate::model::ShiftProblem;

pub async fn save_problem_detail(shift_problem :&ShiftProblem) -> Result<Uuid,Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let result = client.post("http://127.0.0.1:8080/api/sp/save")
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
