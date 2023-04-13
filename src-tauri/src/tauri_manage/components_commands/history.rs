use std::error::Error;

use chrono::NaiveDate;
use errc::{
    config::AppState,
    memory::{
        department::find_department_name_by_id,
        employee::{
            find_4_employees, find_all_employees_names, find_employee_name_by_id,
            find_employees_by_name,
        },
        machine::{find_4_machines, find_all_machines, find_machines_by_name},
        problem::{
            find_department_all_problems, find_department_full_problems_by_name,
            find_problem_by_id, find_problems_by_department_id,
        },
        shift::{
            find_department_shift_id, find_last_21_shifts, find_shifts_after, find_shifts_before,
            find_shifts_between,
        },
        spare_part::{find_4_spare_parts, find_all_spare_parts, find_spare_parts_by_name},
    },
    translator::{translate_date, translate_order},
};
use rec::model::{name::Name, problem::Problem, shift::ClientDbShift};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Day {
    pub date: Vec<String>,
    //             shift_id  order
    pub shifts: Vec<(String, String)>,
}

impl Day {
    fn new(shifts: Vec<ClientDbShift>) -> Vec<Self> {
        shifts
            .into_iter()
            .group_by(|shift| shift.shift_date.clone())
            .into_iter()
            .map(|day_shifts| Day {
                date: translate_date(day_shifts.0),
                shifts: day_shifts
                    .1
                    .map(|shift| {
                        (
                            shift.id,
                            translate_order(&serde_json::from_str(&shift.shift_order).unwrap()),
                        )
                    })
                    .collect(),
            })
            .collect()
    }
}

async fn get_department_days(
    pool: &Pool<Sqlite>,
    department_id: String,
    begin: Option<NaiveDate>,
    end: Option<NaiveDate>,
) -> Result<Vec<Day>, Box<dyn Error>> {
    let mut shifts = match (begin, end) {
        (Some(begin), Some(end)) => find_shifts_between(pool, begin, end).await?,
        (Some(begin), None) => find_shifts_after(pool, begin).await?,
        (None, Some(end)) => find_shifts_before(pool, end).await?,
        (None, None) => find_last_21_shifts(pool).await?,
    };

    for shift in &mut shifts {
        let id = find_department_shift_id(pool, &department_id, &shift.id).await?;
        shift.id = id;
    }

    Ok(Day::new(shifts))
}

#[tauri::command]
pub async fn search_shifts(
    app_state: tauri::State<'_, AppState>,
    department_id: Uuid,
    begin: Option<NaiveDate>,
    end: Option<NaiveDate>,
) -> Result<Vec<Day>, String> {
    match get_department_days(&app_state.pool, department_id.to_string(), begin, end).await {
        Ok(days) => Ok(days),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn search_problem(
    app_state: tauri::State<'_, AppState>,
    department_id: Uuid,
    name: Option<String>,
) -> Result<Vec<Name>, String> {
    if let Some(name) = name {
        if name == "*" {
            return match find_department_all_problems(&app_state.pool, department_id.to_string())
                .await
            {
                Ok(days) => Ok(days),
                Err(err) => Err(err.to_string()),
            };
        }
        match find_department_full_problems_by_name(
            &app_state.pool,
            department_id.to_string(),
            &name.trim(),
        )
        .await
        {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_problems_by_department_id(&app_state.pool, department_id.to_string()).await {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProblemProfile {
    writer_name: String,
    department_name: String,
    title: String,
    description: String,
}

#[tauri::command]
pub async fn profile_problem(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<ProblemProfile, String> {
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
    Ok(ProblemProfile {
        department_name,
        writer_name,
        title,
        description,
    })
}

#[tauri::command]
pub async fn search_parts(
    app_state: tauri::State<'_, AppState>,
    name: Option<String>,
) -> Result<Vec<Name>, String> {
    if let Some(name) = name {
        if name == "*" {
            return match find_all_spare_parts(&app_state.pool).await {
                Ok(days) => Ok(days),
                Err(err) => Err(err.to_string()),
            };
        }
        match find_spare_parts_by_name(&app_state.pool, &name.trim(), vec![]).await {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_4_spare_parts(&app_state.pool, vec![]).await {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[tauri::command]
pub async fn search_machines(
    app_state: tauri::State<'_, AppState>,
    name: Option<String>,
) -> Result<Vec<Name>, String> {
    if let Some(name) = name {
        if name == "*" {
            return match find_all_machines(&app_state.pool).await {
                Ok(days) => Ok(days),
                Err(err) => Err(err.to_string()),
            };
        }
        match find_machines_by_name(&app_state.pool, &name.trim(), vec![]).await {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_4_machines(&app_state.pool, vec![]).await {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[tauri::command]
pub async fn search_employees(
    app_state: tauri::State<'_, AppState>,
    name: Option<String>,
) -> Result<Vec<Name>, String> {
    if let Some(name) = name {
        if name == "*" {
            return match find_all_employees_names(&app_state.pool).await {
                Ok(days) => Ok(days),
                Err(err) => Err(err.to_string()),
            };
        }
        match find_employees_by_name(&app_state.pool, &name.trim(), vec![]).await {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_4_employees(&app_state.pool, vec![]).await {
            Ok(days) => Ok(days),
            Err(err) => Err(err.to_string()),
        }
    }
}
