#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::Mutex, collections::HashMap};

use chrono::NaiveTime;
use errc::{
  model::{Employee, ProblemDetail, Probelm, Machine, SparePart, ShiftProblem, Ids},
  timer::{get_relative_now, get_current_date, get_current_order, get_shift_borders, ShiftOrder},
  translator::{
    translate_date,
    translate_order
  },
  api::{auth::login_req,
        for_selection::{
          all_employees,
          Name,
          all_problems,
          all_machines,
          all_spare_parts
        },
        persistence, fetching::{
          fetch_problem_by_id,
          fetch_machine_by_id,
          fetch_spare_part_by_id,
          fetch_employee_by_id,
          fetch_current_problem_detail,
          WriterAndShiftIds
        }
  }
};
use uuid::Uuid;

#[tauri::command]
fn update_current_shift(state : tauri::State<'_,Mutex<Option<(String,Vec<String>)>>>,
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
fn current_shift(state : tauri::State<'_,Mutex<Option<(String,Vec<String>)>>>) -> Result<(String,Vec<String>),String> {
  match &*state.lock().unwrap() {
    Some((order,date)) => Ok((order.clone(),date.clone())),
    None               => Err("لم يتم تحديث التاريخ".to_owned())
  }
}

#[tauri::command]
fn current_shift_borders(state : tauri::State<'_,Mutex<(Option<NaiveTime>,Option<NaiveTime>)>>) -> Result<(NaiveTime, NaiveTime), String> {
  if let (Some(begin),Some(end)) = &*state.lock().unwrap() {
      Ok((begin.to_owned(),end.to_owned()))
  } else {
      Err("مشكلة في تحديث الوقت".to_string())
  }
}

#[tauri::command]
async fn login(state : tauri::State<'_,Mutex<Option<(Employee,Uuid)>>>,card_id: i16,password: String) -> Result<(),String> {
  match login_req(card_id, password).await {
    Ok((employee,id)) => {
      *state.lock().unwrap() = Some((employee,id));
      Ok(())
    },
    Err(_)     => Err("فشلت عملية تسجيل الدخول".to_string())
  }
}

#[tauri::command]
async fn save_problem_detail(problem_detail : ProblemDetail,department_id : Uuid,
        state : tauri::State<'_,Mutex<HashMap<Uuid,Vec<ShiftProblem>>>>) -> Result<ShiftProblem,String> {
  let shift_problem = ShiftProblem::new(problem_detail);
  match persistence::save_problem_detail(&shift_problem).await {
    Ok(id)   => {
      let shift_problem = ShiftProblem {id : Some(id), ..shift_problem};
      let s = &mut *state.lock().unwrap();
      match s.get_mut(&department_id) {
        Some(problems) => {problems.push(shift_problem.clone())},
        None           => {
          let mut problems = Vec::new();
          problems.push(shift_problem.clone());
          s.insert(department_id, problems);
        }
      }
      Ok(shift_problem)
    },
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
fn get_current_shift_problems(state : tauri::State<'_,Mutex<HashMap<Uuid,Vec<ShiftProblem>>>>,
                              department_id : Uuid) -> Result<Vec<ShiftProblem>,String> {
  match state.lock().unwrap().get(&department_id) {
    Some(problems)   => Ok(problems.to_vec()),
    None => Err("empty".to_string())
  }
}

#[tauri::command]
async fn update_current_shift_problems(state : tauri::State<'_,Mutex<HashMap<Uuid,Vec<ShiftProblem>>>>,
                                       ids : Ids) -> Result<(),String> {
  let Ids{writer_id,shift_id,department_id} = ids;
  match fetch_current_problem_detail(WriterAndShiftIds{writer_id,shift_id}).await {
    Ok(problems)   => {
      let s = &mut *state.lock().unwrap();
      s.insert(department_id, problems);
      Ok(())
    },
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
async fn get_problem_by_id(id : Uuid) -> Result<Probelm,String> {
  match fetch_problem_by_id(id).await {
    Ok(problem)   => Ok(problem),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
async fn get_machine_by_id(id : Uuid) -> Result<Machine,String> {
  match fetch_machine_by_id(id).await {
    Ok(mac)   => Ok(mac),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
async fn get_spare_part_by_id(id : Uuid) -> Result<SparePart,String> {
  match fetch_spare_part_by_id(id).await {
    Ok(s)   => Ok(s),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
async fn get_employee_by_id(id : Uuid) -> Result<Employee,String> {
  match fetch_employee_by_id(id).await {
    Ok(e)   => Ok(e),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
async fn check_login(state : tauri::State<'_,Mutex<Option<(Employee,Uuid)>>>) -> Result<(Employee,Uuid),String> {
  match &*state.lock().unwrap() {
    Some((employee,id)) => Ok((employee.clone(),id.clone())),
    None     => Err("تحتاج الي تسجيل الدخول من جديد".to_string())
  }
}

#[tauri::command]
fn logout(state : tauri::State<'_,Mutex<Option<(Employee,Uuid)>>>) {
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
    .manage(Mutex::new(order.clone()))
    .manage(Mutex::new(get_shift_borders(order)))
    .manage(Mutex::new(None::<(Employee,Uuid)>))
    .manage(Mutex::new(None::<(String,Vec<String>)>))
    .manage(Mutex::new(HashMap::<Uuid,Vec<ShiftProblem>>::new()))
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
      update_current_shift_problems,
      get_current_shift_problems,
      problems_selection,
      machines_selection,
      employees_selection,
      spare_parts_selection,
      save_problem_detail,
      get_employee_by_id,
      get_problem_by_id,
      get_spare_part_by_id,
      get_machine_by_id,
    ])
    .run(tauri::generate_context!())?;
  Ok(())
}
