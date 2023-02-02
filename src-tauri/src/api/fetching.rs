use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::model::{ProblemDetail, Employee, Machine, Probelm, SparePart};

#[derive(Serialize,Deserialize)]
pub struct WriterAndShiftIds{
  writer_id : Uuid,
  shift_id  : Uuid
}

pub async fn fetch_current_problem_detail(writer_shift_ids : WriterAndShiftIds) -> Result<Vec<ProblemDetail>,Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let result = client.get("http://127.0.0.1:8080/api/sp/cproblems")
      .json(&writer_shift_ids)
      .send()
      .await?
      .json::<Option<Vec<ProblemDetail>>>()
      .await?;

  match result {
    Some(problems) => Ok(problems),
    None     => Err("not found".into())
  }
}

pub async fn fetch_employee_by_id(id : Uuid) -> Result<Employee,Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let result = client.post("http://127.0.0.1:8080/api/emp/emp")
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

pub async fn fetch_problem_by_id(id : Uuid) -> Result<Probelm,Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let result = client.post("http://127.0.0.1:8080/api/problem/problem")
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

pub async fn fetch_spare_part_by_id(id : Uuid) -> Result<SparePart,Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let result = client.post("http://127.0.0.1:8080/api/spare-part/part")
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

pub async fn fetch_machine_by_id(id : Uuid) -> Result<Machine,Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let result = client.post("http://127.0.0.1:8080/api/machine/machine")
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
