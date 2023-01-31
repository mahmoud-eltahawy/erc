use uuid::Uuid;

use crate::model::{Cred, Employee};

pub async fn login_req(card_id: i16,password: String) -> Result<(Employee,Uuid),Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let result = client.post("http://127.0.0.1:8080/api/emp/login")
      .json(&Cred{card_id,password})
      .send()
      .await?
      .json::<Option<(Employee,Uuid)>>()
      .await?;

  match result {
      Some((employee,id)) => Ok((employee,id)),
      None    => Err("failed".into())
  }
}
