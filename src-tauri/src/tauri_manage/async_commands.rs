use std::{sync::Mutex, collections::HashMap};

use bcrypt::BcryptResult;
use errc::{
  api::{persistence,
    fetching::fetch_current_problem_detail,
    employee::fetch_employee_by_id,
    shift::save_shift_or,
    problem::save_problem
  },
  config::AppState,
  memory::{
    employee::find_employee_by_card,
    shift::find_shift_by,
    problem::{
      find_problems_by_department_id,
      find_problem_by_id
    }, spare_part::find_spare_part_by_id, machine::find_machine_by_id

  }, syncing::upgrade
};
use rec::{model::{employee::{Employee, ClientEmployee},
                 problem::{Probelm, ClientProblem},
                 shift_problem::{MinimamlShiftProblem, ProblemDetail, WriterAndShiftIds},
                 machine::ClientMachine,
                 spare_part::ClientSparePart,
                 name::Name, shift::{Shift, DateOrder}},
          timer::{get_relative_now, get_current_date, get_current_order}};
use uuid::Uuid;

use super::models::Ids;

fn  verify_password(password : String,hash : &str) -> BcryptResult<bool>{
  bcrypt::verify(password, hash)
}

async fn get_or_save_shift(app_state : &AppState) -> Option<Shift>{
  let now = get_relative_now();
  let date = get_current_date(now);
  let order = get_current_order(now);
  if let Some(date) = date {
    let order = order as i16;
    match find_shift_by(&app_state.pool, DateOrder{date,order}).await {
        Some(shift) => Shift::new(shift),
        None        =>{
          match save_shift_or(app_state).await{
            Ok(shift) =>Some(shift),
            Err(_)    => None
          }
        }
      }
    } else {
    None
  }
}

#[tauri::command]
pub async fn login(emp_and_uuid : tauri::State<'_,Mutex<Option<(ClientEmployee,Uuid)>>>,
               app_state : tauri::State<'_,AppState>,
               card_id: i16,password: String) -> Result<(),String> {
  let failure = Err("فشلت عملية تسجيل الدخول".to_string());

  let employee = match find_employee_by_card(&app_state.pool, card_id).await{
    Ok(e) => e,
    Err(_) => return failure
  };


  match verify_password(password, &employee.password) {
    Ok(result) => if result {
        if let Some(shift) = get_or_save_shift(&app_state).await {
          *emp_and_uuid.lock().unwrap() = Some((employee,shift.id));
          match upgrade(&app_state).await{
            Ok(_) => Ok(()),
            Err(_) => failure
          }
        } else {
          return failure
        }
      } else {
        return failure
      },
    Err(_)   => failure
  }
}

#[tauri::command]
pub async fn define_problem(app_state : tauri::State<'_,AppState>,
                        writer_id : Uuid,
                        department_id : Uuid,
                        title : String,
                        description : String) -> Result<Uuid,String> {
  let id = Uuid::new_v4();
  let problem = Probelm{id,writer_id,department_id,title,description};
  match save_problem(&app_state,&problem).await {
    Ok(_)   =>{},
    Err(err) => return Err(err.to_string())
  };
  match upgrade(&app_state).await{
    Ok(()) => Ok(id),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
pub async fn save_problem_detail(problem_detail : ProblemDetail,department_id : Uuid,
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
pub async fn update_current_shift_problems(
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
pub async fn get_problem_by_id(app_state : tauri::State<'_,AppState>,
  id : Uuid) -> Result<ClientProblem,String> {
  match find_problem_by_id(&app_state.pool,id.to_string()).await {
    Ok(problem)   => Ok(problem),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
pub async fn get_machine_by_id(app_state : tauri::State<'_,AppState>,
  id : Uuid) -> Result<ClientMachine,String> {
  match find_machine_by_id(&app_state.pool,id.to_string()).await {
    Ok(mac)   => Ok(mac),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
pub async fn get_spare_part_by_id(app_state : tauri::State<'_,AppState>,
  id : Uuid) -> Result<ClientSparePart,String> {
  match find_spare_part_by_id(&app_state.pool,id.to_string()).await {
    Ok(s)   => Ok(s),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
pub async fn get_employee_by_id(app_state : tauri::State<'_,AppState>,
  id : Uuid) -> Result<Employee,String> {
  match fetch_employee_by_id(&app_state,id).await {
    Ok(e)   => Ok(e),
    Err(err) => Err(err.to_string())
  }
}

#[tauri::command]
pub async fn problems_selection(app_state : tauri::State<'_,AppState>,
                            department_id : Uuid) -> Result<Vec<Name>,String> {
  match find_problems_by_department_id(&app_state.pool,department_id.to_string()).await {
    Ok(p) => Ok(p.into_iter().map(|p| Name::build_problem(p)).collect()),
    Err(err)=> Err(err.to_string())
  }
}
