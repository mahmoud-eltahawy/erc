use rec::crud_sync::CudVersion;

use crate::config::AppState;

pub async fn fetch_updates(app_state : &AppState,
        current_version : u64) -> Result<Vec<CudVersion>,Box<dyn std::error::Error>> {
  let origin = &app_state.origin;
  let client = reqwest::Client::new();
  let result = client.post(format!("{origin}/api/sync/update"))
      .json(&current_version)
      .send()
      .await?
      .json::<Vec<CudVersion>>()
      .await?;

  Ok(result)
}
