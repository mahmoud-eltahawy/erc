use std::error::Error;

use chrono::NaiveDate;
use errc::{config::AppState, memory::shift::{find_shifts_between, find_department_shift_id, find_last_21_shifts}, translator::{translate_order, translate_date}};
use rec::{model::shift::ClientDbShift, timer::ShiftOrder};

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
