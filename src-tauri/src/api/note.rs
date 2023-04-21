use std::error::Error;

use rec::model::note::{Note, ShiftNote};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_note_to_problem(
    app_state: &AppState,
    note: &Note<Uuid>,
) -> Result<(), Box<dyn Error>> {
    let origin = &app_state.origin;
    let req = reqwest::Client::new()
        .post(format!("{origin}/note/problem"))
        .json(note)
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        _ => Err("server Error".into()),
    }
}

pub async fn save_note_to_shift(
    app_state: &AppState,
    note: &ShiftNote<Uuid>,
) -> Result<(), Box<dyn Error>> {
    let origin = &app_state.origin;
    let req = reqwest::Client::new()
        .post(format!("{origin}/note/shift"))
        .json(note)
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        _ => Err("server Error".into()),
    }
}

pub async fn update_problem_note(
    app_state: &AppState,
    note: &Note<Uuid>,
) -> Result<(), Box<dyn Error>> {
    let origin = &app_state.origin;
    let req = reqwest::Client::new()
        .put(format!("{origin}/note/problem"))
        .json(note)
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        _ => Err("server Error".into()),
    }
}

pub async fn update_shift_note(
    app_state: &AppState,
    note: &ShiftNote<Uuid>,
) -> Result<(), Box<dyn Error>> {
    let origin = &app_state.origin;
    let req = reqwest::Client::new()
        .put(format!("{origin}/note/shift"))
        .json(note)
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        _ => Err("server Error".into()),
    }
}

pub async fn delete_problem_note(app_state: &AppState, id: &Uuid) -> Result<(), Box<dyn Error>> {
    let origin = &app_state.origin;
    let req = reqwest::Client::new()
        .delete(format!("{origin}/note/{id}/problem"))
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        _ => Err("server Error".into()),
    }
}

pub async fn delete_shift_note(app_state: &AppState, id: &Uuid) -> Result<(), Box<dyn Error>> {
    let origin = &app_state.origin;
    let req = reqwest::Client::new()
        .delete(format!("{origin}/note/{id}/shift"))
        .send()
        .await?;

    match req.status() {
        StatusCode::OK => Ok(()),
        _ => Err("server Error".into()),
    }
}
