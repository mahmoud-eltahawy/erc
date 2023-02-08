#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::Mutex, collections::HashMap, env};

use chrono::NaiveTime;
use dotenv::dotenv;
use rec::{
  timer::{
    get_relative_now,
    get_current_date,
    get_current_order,
    get_shift_borders,
    ShiftOrder},
  model::{
    employee::Employee,
    problem::Probelm,
    machine::Machine,
    spare_part::SparePart,
    name::Name,
    shift_problem::{
      MinimamlShiftProblem,
      ProblemDetail
    }
  }
};
use errc::{
  translator::{
    translate_date,
    translate_order
  },
  api::{auth::login_req,
        for_selection::{
          all_employees,
          all_problems,
          all_machines,
          all_spare_parts
        },
        persistence::{self, save_problem}, fetching::{
          fetch_problem_by_id,
          fetch_machine_by_id,
          fetch_spare_part_by_id,
          fetch_employee_by_id,
          fetch_current_problem_detail,
          WriterAndShiftIds
        }
  }, config::AppState
};
use serde::{Serialize, Deserialize};
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
async fn login(emp_and_uuid : tauri::State<'_,Mutex<Option<(Employee,Uuid)>>>,
               app_state : tauri::State<'_,AppState>,
               card_id: i16,password: String) -> Result<(),String> {
  match login_req(&app_state,card_id, password).await {
    Ok((employee,id)) => {
      *emp_and_uuid.lock().unwrap() = Some((employee,id));
      Ok(())
    },
    Err(_)     => Err("فشلت عملية تسجيل الدخول".to_string())
  }
}

#[tauri::command]
async fn define_problem(state : tauri::State<'_,Problems>,
                        app_state : tauri::State<'_,AppState>,
                        title : String,description : String) -> Result<Option<Uuid>,String> {
  let id = Uuid::new_v4();
  let problem = Probelm{id,title,description};
  match save_problem(&app_state,&problem).await {
    Ok(well)   => {
      if well {
        let s = &mut *state.0.lock().unwrap();
        s.push(Name {id : Some(id), name: problem.title });
        Ok(Some(id))
      } else {
        Ok(None)
      }
    },
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
async fn save_problem_detail(problem_detail : ProblemDetail,department_id : Uuid,
                             app_state : tauri::State<'_,AppState>,
        state : tauri::State<'_,Mutex<HashMap<Uuid,
                          Vec<MinimamlShiftProblem>>>>) -> Result<MinimamlShiftProblem,String> {
  let shift_problem = MinimamlShiftProblem::new(problem_detail);
  match persistence::save_problem_detail(&app_state,&shift_problem).await {
    Ok(id)   => {
      let shift_problem = MinimamlShiftProblem {id : Some(id), ..shift_problem};
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
fn get_current_shift_problems(state : tauri::State<'_,Mutex<HashMap<Uuid,Vec<MinimamlShiftProblem>>>>,
                              department_id : Uuid) -> Result<Vec<MinimamlShiftProblem>,String> {
  match state.lock().unwrap().get(&department_id) {
    Some(problems)   => Ok(problems.to_vec()),
    None => Err("empty".to_string())
  }
}

#[tauri::command]
async fn update_current_shift_problems(
  state : tauri::State<'_,Mutex<HashMap<Uuid,Vec<MinimamlShiftProblem>>>>,
  app_state : tauri::State<'_,AppState>,ids : Ids) -> Result<(),String> {
  let Ids{writer_id,shift_id,department_id} = ids;
  match fetch_current_problem_detail(&app_state,WriterAndShiftIds{writer_id,shift_id}).await {
    Ok(problems)   => {
      let s = &mut *state.lock().unwrap();
      s.insert(department_id, problems);
      Ok(())
    },
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
async fn get_problem_by_id(app_state : tauri::State<'_,AppState>,
  id : Uuid) -> Result<Probelm,String> {
  match fetch_problem_by_id(&app_state,id).await {
    Ok(problem)   => Ok(problem),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
async fn get_machine_by_id(app_state : tauri::State<'_,AppState>,
  id : Uuid) -> Result<Machine,String> {
  match fetch_machine_by_id(&app_state,id).await {
    Ok(mac)   => Ok(mac),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
async fn get_spare_part_by_id(app_state : tauri::State<'_,AppState>,
  id : Uuid) -> Result<SparePart,String> {
  match fetch_spare_part_by_id(&app_state,id).await {
    Ok(s)   => Ok(s),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
async fn get_employee_by_id(app_state : tauri::State<'_,AppState>,
  id : Uuid) -> Result<Employee,String> {
  match fetch_employee_by_id(&app_state,id).await {
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
fn employees_selection(state : tauri::State<'_,Employees>) -> Vec<Name> {
  let s = &*state.0.lock().unwrap();
  s.to_vec()
}

#[tauri::command]
fn problems_selection(state : tauri::State<'_,Problems>) -> Vec<Name> {
  let s = &*state.0.lock().unwrap();
  s.to_vec()
}

#[tauri::command]
fn machines_selection(state : tauri::State<'_,Machines>) -> Vec<Name> {
  let s = &*state.0.lock().unwrap();
  s.to_vec()
}

#[tauri::command]
fn spare_parts_selection(state : tauri::State<'_,SpareParts>) -> Vec<Name> {
  let s = &*state.0.lock().unwrap();
  s.to_vec()
}
#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
  dotenv().ok();
  launch_tauri().await?;
  Ok(())
}

#[derive(Serialize,Deserialize)]
pub struct Ids{
  pub writer_id     : Uuid,
  pub shift_id      : Uuid,
  pub department_id : Uuid
}

struct Employees(Mutex<Vec<Name>>);
struct Problems(Mutex<Vec<Name>>);
struct Machines(Mutex<Vec<Name>>);
struct SpareParts(Mutex<Vec<Name>>);

async fn launch_tauri() -> Result<(),Box<dyn std::error::Error>>{
  let app_state = AppState::new();
  let relative_now = get_relative_now();
  let order = get_current_order(relative_now);

  let problems = match all_problems(&app_state).await {
    Ok(p) => p,
    Err(err)=> return Err(err.into())
  };

  let employees = match all_employees(&app_state).await {
    Ok(e) => e,
    Err(err)=> return Err(err.into())
  };

  let machines = match all_machines(&app_state).await {
    Ok(m) => m,
    Err(err)=> return Err(err.into())
  };

  let spare_parts = match all_spare_parts(&app_state).await {
    Ok(s) => s,
    Err(err)=> return Err(err.into())
  };

  tauri::Builder::default()
    .manage(app_state)
    .manage(Mutex::new(relative_now))
    .manage(Mutex::new(order.clone()))
    .manage(Mutex::new(get_shift_borders(order)))
    .manage(Mutex::new(None::<(Employee,Uuid)>))
    .manage(Mutex::new(None::<(String,Vec<String>)>))
    .manage(Mutex::new(HashMap::<Uuid,Vec<MinimamlShiftProblem>>::new()))
    .manage(Employees(Mutex::new(employees)))
    .manage(Problems(Mutex::new(problems)))
    .manage(Machines(Mutex::new(machines)))
    .manage(SpareParts(Mutex::new(spare_parts)))
    .invoke_handler(tauri::generate_handler![
      login,
      logout,
      check_login,
      current_shift,
      current_shift_borders,
      update_current_shift,
      update_current_shift_problems,
      get_current_shift_problems,
      define_problem,
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
