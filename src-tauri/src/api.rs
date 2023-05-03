use rec::{
    crud_sync::Version,
    model::{shift::Shift, TableRequest, TableResponse},
};
use uuid::Uuid;

use crate::config::AppState;

pub async fn main_entry(
    app_state: &AppState,
    table_req: TableRequest,
) -> Result<TableResponse, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let res = reqwest::Client::new()
        .post(format!("{origin}/app/"))
        .json(&table_req)
        .send()
        .await?
        .json::<TableResponse>()
        .await?;
    Ok(res)
}

pub async fn fetch_shift(
    app_state: &AppState,
    id: &Uuid,
) -> Result<Shift, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let res = reqwest::Client::new()
        .get(format!("{origin}/app/{id}"))
        .send()
        .await?
        .json::<Shift>()
        .await?;
    Ok(res)
}

pub async fn fetch_last_versions(
    app_state: &AppState,
    (cd, updates): (i32, i32),
) -> Result<Vec<Version>, Box<dyn std::error::Error>> {
    let origin = &app_state.origin;
    let res = reqwest::Client::new()
        .get(format!("{origin}/app/{cd}/{updates}"))
        .send()
        .await?
        .json::<Vec<Version>>()
        .await?;
    Ok(res)
}
