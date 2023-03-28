use std::error::Error;
use rec::model::permissions::{Permissions, PermissionsNames};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::config::AppState;

fn get_permission_target(permission : &PermissionsNames) -> String {
  match permission {
    PermissionsNames::WriteDepartmentProblem
      => "write_department_problem".to_string(),
    PermissionsNames::ReadDepartmentProblems
      => "read_department_problems".to_string(),
    PermissionsNames::ModifyDepartmentProblems
      => "modify_department_problems".to_string(),
    PermissionsNames::DefineProblem
      => "define_problem".to_string(),
    PermissionsNames::AccessHistoryDepartmentProblems
      => "access_history_department_problems".to_string(),
    PermissionsNames::AccessHistoryAllDepartmentsProblems
      => "access_history_all_departments_problems".to_string(),
    PermissionsNames::AccessHistoryDepartmentDepartmentProblems
      => "access_history_department_department_problems".to_string(),
    PermissionsNames::AccessHistoryAllDepartmentsDepartmentProblems
      => "access_history_all_departments_department_problems".to_string(),
    PermissionsNames::AccessHistoryMachines
      => "access_history_machines".to_string(),
    PermissionsNames::AccessHistorySpareParts
      => "access_history_spare_parts".to_string(),
    PermissionsNames::AccessHistoryEmployees
      => "access_history_employees".to_string(),
  }
}

pub async fn save_permissions(app_state : &AppState,permissions : &Permissions) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .post(format!("{origin}/per/"))
    .json(permissions)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn allow_permission(app_state : &AppState,id : Uuid,permission : &PermissionsNames) -> Result<(),Box<dyn Error>> {
  let origin     = &app_state.origin;
  let permission = get_permission_target(permission);
  let id = id.to_string();
  let req = reqwest::Client::new()
    .get(format!("{origin}/per/{id}/{permission}/allow"))
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn forbid_permission(app_state : &AppState,id : Uuid,permission : &PermissionsNames) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let permission = get_permission_target(permission);
  let id = id.to_string();
  let req = reqwest::Client::new()
    .get(format!("{origin}/per/{id}/{permission}/forbid"))
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}

pub async fn update_department(app_state : &AppState,permissions : &Permissions) -> Result<(),Box<dyn Error>> {
  let origin = &app_state.origin;
  let req = reqwest::Client::new()
    .put(format!("{origin}/per/"))
    .json(permissions)
    .send()
    .await?;

  match req.status() {
    StatusCode::OK => Ok(()),
    _              => Err("server Error".into())
  }
}
