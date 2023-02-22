use std::{sync::Mutex, error::Error};

use bcrypt::BcryptResult;
use errc::{
  api::{
    shift::save_shift_or,
    problem::save_problem,
    relations::shift_problem::{
      save_problem_to_shift_problem,
      save_spare_part_to_shift_problem
    },
    shift_problem::save_shift_problem,
    note::save_note_to_problem
  },
  config::AppState,
  memory::{
    employee::{find_employee_by_card, find_employee_by_id},
    shift::find_shift_by,
    problem::find_problem_by_id,
    spare_part::find_spare_part_by_id,
    machine::find_machine_by_id,
    relations::shift_problems::{
      fetch_shift_problem_problems,
      fetch_shift_problem_spare_parts
    },
    note::fetch_shift_problem_note,
    shift_problem::find_shift_shift_problems
  }, syncing::upgrade
};
use rec::{model::{employee::ClientEmployee,
                 problem::{Probelm, ClientProblem},
                 shift_problem::{MinimamlShiftProblem, ProblemDetail, ClientMinimamlShiftProblem},
                 machine::ClientMachine,
                 spare_part::ClientSparePart,
                 shift::{Shift, DateOrder},
                 relations::{ShiftProblemProblem, ShiftProblemSparePart},
                 note::{Note, DbNote}},
          timer::{get_relative_now, get_current_date, get_current_order}};
use uuid::Uuid;

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
        Some(shift) => Shift::get(shift),
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
               card_id: i64,password: String) -> Result<(),String> {
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

async fn save_minimal_shift_problem(app_state : &AppState,
              minimal_shift_problem : MinimamlShiftProblem) -> Result<MinimamlShiftProblem,Box<dyn Error>>{
  let (shift_problem,problems,parts,note) = minimal_shift_problem.destruct();
  save_shift_problem(app_state, &shift_problem).await?;
  let shift_problem_id = shift_problem.id;
  for problem_id in problems.clone() {
    save_problem_to_shift_problem(app_state,
              &ShiftProblemProblem{
                problem_id,
                shift_problem_id
              }).await?;
  }

  if let Some(parts_ids) = parts.clone(){
    for spare_part_id in parts_ids {
      save_spare_part_to_shift_problem(app_state,
              &ShiftProblemSparePart{
                spare_part_id,
                shift_problem_id
              }).await?;
    }
  }

  if let Some(note) = note.clone() {
    let Note{id,content} = note;
    let shift_problem_id = Some(shift_problem_id);
    save_note_to_problem(app_state,
          &DbNote{
            id,content,
            shift_problem_id,
            shift_id:None
          }).await?;
  }

  Ok(MinimamlShiftProblem::construct((shift_problem,problems,parts,note)))
}

#[tauri::command]
pub async fn save_problem_detail(problem_detail : ProblemDetail,
                              app_state : tauri::State<'_,AppState>) -> Result<(),String> {
  let shift_problem = MinimamlShiftProblem::new(problem_detail);
  match save_minimal_shift_problem(&app_state,shift_problem).await {
    Ok(_)    => {},
    Err(err) => return Err(err.to_string())
  };
  match upgrade(&app_state).await{
    Ok(_)    => Ok(()),
    Err(err) => Err(err.to_string())
  }
}

async fn fetch_minimal_shift_problem_by_shift_id(app_state : &AppState,
              shift_id : Uuid) -> Result<Vec<ClientMinimamlShiftProblem>,Box<dyn Error>>{

  let shift_problems = find_shift_shift_problems(&app_state.pool,
                                        shift_id.to_string()).await?;
  let mut result = Vec::new();
  for sp in shift_problems{
    let problems = fetch_shift_problem_problems(&app_state.pool, &sp.id).await?;
    let parts    = fetch_shift_problem_spare_parts(&app_state.pool, &sp.id).await?;
    let parts    = if parts.is_empty() {None} else {Some(parts)};
    let note     = fetch_shift_problem_note(&app_state.pool, &sp.id).await;
    result.push(ClientMinimamlShiftProblem::construct((sp,problems,parts,note)));
  }
  Ok(result)
}

#[tauri::command]
pub async fn get_current_shift_problems(app_state : tauri::State<'_,AppState>,
          shift_id : Uuid) -> Result<Vec<ClientMinimamlShiftProblem>,String> {
  match fetch_minimal_shift_problem_by_shift_id(&app_state,
                                      shift_id).await {
    Ok(problems)   => Ok(problems.to_vec()),
    Err(_) => Err("empty".to_string())
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
  id : Uuid) -> Result<ClientEmployee,String> {
  match find_employee_by_id(&app_state.pool,id.to_string()).await {
    Ok(e)   => Ok(e),
    Err(err) => Err(err.to_string())
  }
}
