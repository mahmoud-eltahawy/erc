use uuid::Uuid;

use crate::{model::{Cred, Employee}, config::AppState};

pub async fn login_req(app_state : &AppState,card_id: i16,password: String) -> Result<(Employee,Uuid),Box<dyn std::error::Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/emp/login"))
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
