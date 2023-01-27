#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use errc::{
  model::Employee,
  timer::{get_relative_now, get_current_date, get_current_order, get_current_shift_borders},
  translator::{
    translate_date,
    translate_order
  },
  api::{auth::login_req, for_selection::{all_employees, Name, all_problems, all_machines, all_spare_parts}}
};

#[tauri::command]
fn update_current_shift(state : tauri::State<'_,Mutex<Option<(String,Vec<String>)>>>,
now : tauri::State<'_,Mutex<i64>>,order : tauri::State<'_,Mutex<u8>>) -> Result<(),String> {
  let now = *now.lock().unwrap();
  match get_current_date(now) {
    Some(date) => {
      *state.lock().unwrap() = Some((translate_order(*order.lock().unwrap()),translate_date(date)));
      Ok(())
    },
    None               => Err("مشكلة داخلية في تحديث التاريخ".to_owned())
  }
}

#[tauri::command]
fn current_shift(state : tauri::State<'_,Mutex<Option<(String,Vec<String>)>>>) -> Result<(String,Vec<String>),String> {
  match &*state.lock().unwrap() {
    Some((order,date)) => Ok((order.clone(),date.clone())),
    None               => Err("لم يتم تحديث التاريخ".to_owned())
  }
}

#[tauri::command]
fn current_shift_borders(state : tauri::State<'_,Mutex<Option<(String,String)>>>) -> Result<(String, String), String> {
  match &*state.lock().unwrap() {
    Some((begin,end)) => Ok((begin.to_owned(),end.to_owned())),
    None              => Err("مشكلة في تحديث الوقت".to_string())
  }
}

#[tauri::command]
async fn login(state : tauri::State<'_,Mutex<Option<Employee>>>,card_id: i16,password: String) -> Result<(),String> {
  match login_req(card_id, password).await {
    Ok(employee) => {
      *state.lock().unwrap() = Some(employee);
      Ok(())
    },
    Err(_)     => Err("فشلت عملية تسجيل الدخول".to_string())
  }
}

#[tauri::command]
async fn check_login(state : tauri::State<'_,Mutex<Option<Employee>>>) -> Result<Employee,String> {
  match &*state.lock().unwrap() {
    Some(employee) => Ok(employee.clone()),
    None     => Err("تحتاج الي تسجيل الدخول من جديد".to_string())
  }
}

#[tauri::command]
fn logout(state : tauri::State<'_,Mutex<Option<Employee>>>) {
  *state.lock().unwrap() = None;
}

#[tauri::command]
async fn update_employees_selection(state : tauri::State<'_,Employees>) -> Result<(),String> {
  match all_employees().await {
    Ok(e) => {
      *state.0.lock().unwrap() = Some(e);
      Ok(())
    },
    Err(err)=> Err(err.to_string())
  }
}

#[tauri::command]
async fn update_problems_selection(state : tauri::State<'_,Problems>) -> Result<(),String> {
  match all_problems().await {
    Ok(p) => {
      *state.0.lock().unwrap() = Some(p);
      Ok(())
    },
    Err(err)=> Err(err.to_string())
  }
}

#[tauri::command]
async fn update_machines_selection(state : tauri::State<'_,Machines>) -> Result<(),String> {
  match all_machines().await {
    Ok(m) => {
      *state.0.lock().unwrap() = Some(m);
      Ok(())
    },
    Err(err)=> Err(err.to_string())
  }
}

#[tauri::command]
async fn update_spare_parts_selection(state : tauri::State<'_,SpareParts>) -> Result<(),String> {
  match all_spare_parts().await {
    Ok(s) => {
      *state.0.lock().unwrap() = Some(s);
      Ok(())
    },
    Err(err)=> Err(err.to_string())
  }
}

#[tauri::command]
async fn employees_selection(state : tauri::State<'_,Employees>) -> Result<Vec<Name>,String> {
  match &*state.0.lock().unwrap() {
    Some(employees) => Ok(employees.clone()),
    None     => Err("لم يتم تحديث الموظفين".to_string())
  }
}

#[tauri::command]
async fn problems_selection(state : tauri::State<'_,Problems>) -> Result<Vec<Name>,String> {
  match &*state.0.lock().unwrap() {
    Some(problems) => Ok(problems.clone()),
    None     => Err("لم يتم تحديث المشاكل".to_string())
  }
}

#[tauri::command]
async fn machines_selection(state : tauri::State<'_,Machines>) -> Result<Vec<Name>,String> {
  match &*state.0.lock().unwrap() {
    Some(machines) => Ok(machines.clone()),
    None     => Err("لم يتم تحديث الماكينات".to_string())
  }
}

#[tauri::command]
async fn spare_parts_selection(state : tauri::State<'_,SpareParts>) -> Result<Vec<Name>,String> {
  match &*state.0.lock().unwrap() {
    Some(spare_parts) => Ok(spare_parts.clone()),
    None     => Err("لم يتم تحديث قطع الغيار".to_string())
  }
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
  launch_tauri().await?;
  Ok(())
}

struct Employees(Mutex<Option<Vec<Name>>>);
struct Problems(Mutex<Option<Vec<Name>>>);
struct Machines(Mutex<Option<Vec<Name>>>);
struct SpareParts(Mutex<Option<Vec<Name>>>);

async fn launch_tauri() -> Result<(),Box<dyn std::error::Error>>{
  let relative_now = get_relative_now();
  let order = get_current_order(relative_now);
  tauri::Builder::default()
    .manage(Mutex::new(relative_now))
    .manage(Mutex::new(order))
    .manage(Mutex::new(get_current_shift_borders(order)))
    .manage(Mutex::new(None::<Employee>))
    .manage(Mutex::new(None::<(String,Vec<String>)>))
    .manage(Employees(Mutex::new(None::<Vec<Name>>)))
    .manage(Problems(Mutex::new(None::<Vec<Name>>)))
    .manage(Machines(Mutex::new(None::<Vec<Name>>)))
    .manage(SpareParts(Mutex::new(None::<Vec<Name>>)))
    .invoke_handler(tauri::generate_handler![
      login,
      logout,
      check_login,
      current_shift,
      current_shift_borders,
      update_current_shift,
      update_employees_selection,
      update_problems_selection,
      update_machines_selection,
      update_spare_parts_selection,
      problems_selection,
      machines_selection,
      employees_selection,
      spare_parts_selection
    ])
    .run(tauri::generate_context!())?;
  Ok(())
}
