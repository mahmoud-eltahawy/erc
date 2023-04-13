use rec::model::department::Department;
use reqwest::StatusCode;
use std::error::Error;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_department(
    app_state: &AppState,
    dep: &Department<Uuid>,
) -> Result<(), Box<dyn Error>> {
    let origin = &app_state.origin;
    let req = reqwest::Client::new()
        .post(format!("{origin}/dep/"))
        .json(dep)
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        _ => Err("server Error".into()),
    }
}

pub async fn update_department(
    app_state: &AppState,
    dep: &Department<Uuid>,
) -> Result<(), Box<dyn Error>> {
    let origin = &app_state.origin;
    let req = reqwest::Client::new()
        .put(format!("{origin}/dep/"))
        .json(dep)
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        _ => Err("server Error".into()),
    }
}

pub async fn delete_department(app_state: &AppState, id: &Uuid) -> Result<(), Box<dyn Error>> {
    let origin = &app_state.origin;
    let req = reqwest::Client::new()
        .delete(format!("{origin}/dep/{id}"))
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        _ => Err("server Error".into()),
    }
}

pub async fn sets_department_boss(app_state: &AppState, id: &Uuid) -> Result<(), Box<dyn Error>> {
    let origin = &app_state.origin;
    let req = reqwest::Client::new()
        .get(format!("{origin}/dep/{id}/boss"))
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        _ => Err("server Error".into()),
    }
}
