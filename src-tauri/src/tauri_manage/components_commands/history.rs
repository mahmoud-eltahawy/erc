use std::error::Error;

use chrono::NaiveDate;
use errc::{
  config::AppState,
  translator::{
    translate_order,
    translate_date,
  },
  memory::{shift::{
      find_shifts_between,
      find_department_shift_id,
      find_last_21_shifts
    }, problem::{
      find_department_full_problems_by_name,
      find_problems_by_department_id,
      find_department_all_problems, find_problem_by_id
    },
    spare_part::{
      find_all_spare_parts,
      find_spare_parts_by_name,
      find_4_spare_parts,
    },
    machine::{
      find_all_machines,
      find_4_machines,
      find_machines_by_name
    },
    employee::{
      find_all_employees_names,
      find_employees_by_name,
      find_4_employees,
      find_employee_name_by_id,
    },
    department::find_department_name_by_id,
  }
};
use rec::{
  model::{
    shift::ClientDbShift,
    name::Name,
    problem::Problem
  },
  timer::ShiftOrder
};

use itertools::Itertools;
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct Day{
    pub date   : Vec<String>,
                    //id     order
    pub shifts : Vec<(String,String)>
}

impl Day {
    fn new(shifts : Vec<ClientDbShift>) -> Vec<Self>{
      shifts.into_iter()
        .group_by(|a| a.shift_date.clone())
        .into_iter()
        .map(|day_shifts|{
           let date = day_shifts.0;
           let date = translate_date(date);
           let shifts : Vec<(String,String)> = day_shifts.1.map(|a|{
             let order : ShiftOrder = serde_json::from_str(&a.shift_order).unwrap();
             let order = translate_order(&order);
             (a.id,order)
           }).collect();
           Day{date,shifts}
        }).collect()
    }
}

async fn get_department_days(pool:&Pool<Sqlite>,
              department_id : String,begin : Option<NaiveDate>,end : Option<NaiveDate>) -> Result<Vec<Day>,Box<dyn Error>>{
  let mut shifts;
  if let (Some(begin),Some(end)) = (begin,end) {
    shifts = find_shifts_between(pool, begin, end).await?;
  }else{
    shifts = find_last_21_shifts(pool).await?;
  }
  for shift in &mut shifts{
    let id = find_department_shift_id(pool, &department_id, &shift.id).await?;
    shift.id = id;
  }
  Ok(Day::new(shifts))
}

#[tauri::command]
pub async fn search_shifts(app_state : tauri::State<'_,AppState>,
       department_id : Uuid,begin : Option<NaiveDate>,end : Option<NaiveDate>) -> Result<Vec<Day>,String> {
    match get_department_days(&app_state.pool, department_id.to_string(), begin, end).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
pub async fn search_problem(app_state : tauri::State<'_,AppState>,
                  department_id : Uuid,name : Option<String>) -> Result<Vec<Name>,String> {
  if let Some(name) = name {
    if name == "*" {
      return match find_department_all_problems(&app_state.pool, department_id.to_string()).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string())
      };
    }
    match find_department_full_problems_by_name(&app_state.pool, department_id.to_string(),&name.trim() ).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
  } else {
    match find_problems_by_department_id(&app_state.pool, department_id.to_string()).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
  }
}

#[derive(Serialize,Deserialize)]
pub struct ProblemProfile{
    writer_name          : String,
    department_name      : String,
    title                : String,
    description          : String
}

#[tauri::command]
pub async fn profile_problem(app_state : tauri::State<'_,AppState>,
                  id : Uuid) -> Result<ProblemProfile,String> {
  let Ok(Problem{id : _,department_id,writer_id,
      title,description}) = find_problem_by_id(&app_state.pool, id.to_string()).await else {
    return Err("err".to_string());
  };
  let Ok(writer_name) = find_employee_name_by_id(&app_state.pool, writer_id).await else {
    return Err("err".to_string());
  };
  let Ok(department_name) = find_department_name_by_id(&app_state.pool, department_id).await else {
    return Err("err".to_string());
  };
  Ok(ProblemProfile { department_name,writer_name,title,description })
}

#[tauri::command]
pub async fn search_parts(app_state : tauri::State<'_,AppState>,
                          name : Option<String>) -> Result<Vec<Name>,String> {
  if let Some(name) = name {
    if name == "*" {
      return match find_all_spare_parts(&app_state.pool).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string())
      };
    }
    match find_spare_parts_by_name(&app_state.pool,&name.trim(),vec![]).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
  } else {
    match find_4_spare_parts(&app_state.pool,vec![]).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
  }
}

#[tauri::command]
pub async fn search_machines(app_state : tauri::State<'_,AppState>,name : Option<String>) -> Result<Vec<Name>,String> {
  if let Some(name) = name {
    if name == "*" {
      return match find_all_machines(&app_state.pool).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string())
      };
    }
    match find_machines_by_name(&app_state.pool,&name.trim(),vec![]).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
  } else {
    match find_4_machines(&app_state.pool,vec![]).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
  }
}

#[tauri::command]
pub async fn search_employees(app_state : tauri::State<'_,AppState>,name : Option<String>) -> Result<Vec<Name>,String> {
  if let Some(name) = name {
    if name == "*" {
      return match find_all_employees_names(&app_state.pool).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string())
      };
    }
    match find_employees_by_name(&app_state.pool,&name.trim(),vec![]).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
  } else {
    match find_4_employees(&app_state.pool,vec![]).await {
      Ok(days) => Ok(days),
      Err(err) => Err(err.to_string())
    }
  }
}
