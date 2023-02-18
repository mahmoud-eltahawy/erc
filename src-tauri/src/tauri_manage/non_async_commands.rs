use std::{sync::Mutex, collections::HashMap};
use chrono::NaiveTime;
use rec::{
  timer::{
    get_current_date,
    ShiftOrder
  },model::{
    employee::ClientEmployee,
    name::Name,
    shift_problem::MinimamlShiftProblem
  }
};
use errc::translator::{
  translate_date,
  translate_order
};
use uuid::Uuid;

use super::models::{Employees, Machines, SpareParts};

#[tauri::command]
pub fn check_login(state : tauri::State<'_,Mutex<Option<(ClientEmployee,Uuid)>>>) -> Result<(ClientEmployee,Uuid),String> {
  match &*state.lock().unwrap() {
    Some((employee,id)) => Ok((employee.clone(),id.clone())),
    None     => Err("تحتاج الي تسجيل الدخول من جديد".to_string())
  }
}

#[tauri::command]
pub fn update_current_shift(state : tauri::State<'_,Mutex<Option<(String,Vec<String>)>>>,
now : tauri::State<'_,Mutex<i64>>,order : tauri::State<'_,Mutex<ShiftOrder>>) -> Result<(),String> {
  let now = *now.lock().unwrap();
  match get_current_date(now) {
    Some(date) => {
      *state.lock().unwrap() = Some((translate_order(&*order.lock().unwrap()),translate_date(date.to_string())));
      Ok(())
    },
    None               => Err("مشكلة داخلية في تحديث التاريخ".to_owned())
  }
}

#[tauri::command]
pub fn current_shift(state : tauri::State<'_,Mutex<Option<(String,Vec<String>)>>>) -> Result<(String,Vec<String>),String> {
  match &*state.lock().unwrap() {
    Some((order,date)) => Ok((order.clone(),date.clone())),
    None               => Err("لم يتم تحديث التاريخ".to_owned())
  }
}

#[tauri::command]
pub fn current_shift_borders(state : tauri::State<'_,Mutex<(Option<NaiveTime>,Option<NaiveTime>)>>) -> Result<(NaiveTime, NaiveTime), String> {
  if let (Some(begin),Some(end)) = &*state.lock().unwrap() {
      Ok((begin.to_owned(),end.to_owned()))
  } else {
      Err("مشكلة في تحديث الوقت".to_string())
  }
}

#[tauri::command]
pub fn get_current_shift_problems(state : tauri::State<'_,Mutex<HashMap<Uuid,Vec<MinimamlShiftProblem>>>>,
                              department_id : Uuid) -> Result<Vec<MinimamlShiftProblem>,String> {
  match state.lock().unwrap().get(&department_id) {
    Some(problems)   => Ok(problems.to_vec()),
    None => Err("empty".to_string())
  }
}

#[tauri::command]
pub fn logout(state : tauri::State<'_,Mutex<Option<(ClientEmployee,Uuid)>>>) {
  *state.lock().unwrap() = None;
}

#[tauri::command]
pub fn employees_selection(state : tauri::State<'_,Employees>) -> Vec<Name> {
  let s = &*state.0.lock().unwrap();
  s.to_vec()
}

#[tauri::command]
pub fn machines_selection(state : tauri::State<'_,Machines>) -> Vec<Name> {
  let s = &*state.0.lock().unwrap();
  s.to_vec()
}

#[tauri::command]
pub fn spare_parts_selection(state : tauri::State<'_,SpareParts>) -> Vec<Name> {
  let s = &*state.0.lock().unwrap();
  s.to_vec()
}
