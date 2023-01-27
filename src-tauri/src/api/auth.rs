use crate::model::{Cred, Employee};

pub async fn login_req(card_id: i16,password: String) -> Result<Employee,Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let result = client.post("http://127.0.0.1:8080/api/emp/login")
      .json(&Cred{card_id,password})
      .send()
      .await?
      .json::<Option<Employee>>()
      .await?;

  match result {
      Some(employee) => Ok(employee),
      None    => Err("failed".into())
  }
}
