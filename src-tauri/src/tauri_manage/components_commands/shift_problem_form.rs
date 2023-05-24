use std::sync::Mutex;

use chrono::{Local, NaiveDateTime, NaiveTime};
use errc::{
    api::main_entry,
    config::AppState,
    memory::{
        employee::{
            find_employees_by_name, find_limit_of_employees, find_shift_existing_employees_ids,
            find_shift_non_existing_employees_ids,
        },
        machine::{find_limit_of_machines, find_machines_by_name},
        note::{fetch_shift_note_by_id, fetch_shift_notes_ids_by_shift_id},
        problem::{find_department_limit_of_problems, find_department_problems_by_name},
        spare_part::{find_limit_of_spare_parts, find_spare_parts_by_name},
    },
    syncing::upgrade,
};
use rec::{
    model::{
        name::Name,
        note::{Note, ShiftNote},
        shift::UpdateDepartmentShift,
        Environment, TableCrud, TableRequest, TableResponse,
    },
    timer::{get_current_order, get_relative_now, get_shift_borders},
};
use tauri::Window;
use uuid::Uuid;

#[tauri::command]
pub async fn problems_selection(
    app_state: tauri::State<'_, AppState>,
    department_id: Uuid,
    name: Option<String>,
    canceled: Vec<String>,
    limit: i64,
) -> Result<Vec<Name>, String> {
    if let Some(name) = name {
        match find_department_problems_by_name(
            &app_state.pool,
            department_id,
            &name.trim(),
            canceled,
            limit,
        )
        .await
        {
            Ok(p) => Ok(p),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_department_limit_of_problems(&app_state.pool, department_id, canceled, limit)
            .await
        {
            Ok(p) => Ok(p),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[tauri::command]
pub async fn employees_selection(
    app_state: tauri::State<'_, AppState>,
    name: Option<String>,
    canceled: Vec<Uuid>,
    limit: i64,
) -> Result<Vec<Name>, String> {
    if let Some(name) = name {
        match find_employees_by_name(&app_state.pool, &name.trim(), canceled, limit).await {
            Ok(e) => Ok(e),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_limit_of_employees(&app_state.pool, canceled, limit).await {
            Ok(e) => Ok(e),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[tauri::command]
pub async fn machines_selection(
    app_state: tauri::State<'_, AppState>,
    name: Option<String>,
    canceled: Vec<String>,
    limit: i64,
) -> Result<Vec<Name>, String> {
    if let Some(name) = name {
        match find_machines_by_name(&app_state.pool, &name.trim(), canceled, limit).await {
            Ok(e) => Ok(e),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_limit_of_machines(&app_state.pool, canceled, limit).await {
            Ok(e) => Ok(e),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[tauri::command]
pub async fn spare_parts_selection(
    app_state: tauri::State<'_, AppState>,
    name: Option<String>,
    canceled: Vec<String>,
    limit: i64,
) -> Result<Vec<Name>, String> {
    if let Some(name) = name {
        match find_spare_parts_by_name(&app_state.pool, &name.trim(), canceled, limit).await {
            Ok(e) => Ok(e),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_limit_of_spare_parts(&app_state.pool, canceled, limit).await {
            Ok(e) => Ok(e),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[tauri::command]
pub async fn shift_existing_employees(
    app_state: tauri::State<'_, AppState>,
    shift_id: Uuid,
) -> Result<Vec<Uuid>, String> {
    match find_shift_existing_employees_ids(&app_state.pool, shift_id).await {
        Ok(e) => Ok(e),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn shift_non_existing_employees(
    app_state: tauri::State<'_, AppState>,
    shift_id: Uuid,
    department_id: Uuid,
) -> Result<Vec<Uuid>, String> {
    match find_shift_non_existing_employees_ids(&app_state.pool, shift_id, department_id).await {
        Ok(e) => Ok(e),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn current_shift_borders() -> Result<(NaiveTime, NaiveTime), String> {
    let relative_now = get_relative_now();
    let order = get_current_order(relative_now);
    match get_shift_borders(order) {
        Some(v) => Ok(v),
        None => Err("shift borders is null".to_string()),
    }
}

#[tauri::command]
pub async fn add_shift_employee(
    app_state: tauri::State<'_, AppState>,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    window: Window,
    employee_id: Uuid,
) -> Result<(), String> {
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
       return Err("null time stamp".into());
   };
    let Some((updater_id, shift_id)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let res = match main_entry(
        &app_state,
        TableRequest::DepartmentShift(TableCrud::Update(Environment {
            target: UpdateDepartmentShift::SaveShiftEmployee(shift_id, employee_id),
            updater_id,
            time_stamp,
        })),
    )
    .await
    {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    match res {
        TableResponse::Done => Ok(upgrade(&app_state, &window).await),
        TableResponse::Err(err) => Err(err.into()),
        _ => unreachable!(),
    }
}

#[tauri::command]
pub async fn remove_shift_employee(
    app_state: tauri::State<'_, AppState>,
    window: Window,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    employee_id: Uuid,
) -> Result<(), String> {
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
       return Err("null time stamp".into());
   };
    let Some((updater_id, shift_id)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let res = match main_entry(
        &app_state,
        TableRequest::DepartmentShift(TableCrud::Update(Environment {
            target: UpdateDepartmentShift::DeleteShiftEmployee(shift_id, employee_id),
            updater_id,
            time_stamp,
        })),
    )
    .await
    {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    match res {
        TableResponse::Done => Ok(upgrade(&app_state, &window).await),
        TableResponse::Err(err) => Err(err.into()),
        _ => unreachable!(),
    }
}

#[tauri::command]
pub async fn save_shift_note(
    app_state: tauri::State<'_, AppState>,
    window: Window,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    content: String,
) -> Result<(), String> {
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
       return Err("null time stamp".into());
   };
    let Some((updater_id, shift_id)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let res = match main_entry(
        &app_state,
        TableRequest::DepartmentShift(TableCrud::Update(Environment {
            target: UpdateDepartmentShift::SaveNote(ShiftNote {
                id: Uuid::new_v4(),
                shift_id,
                content,
            }),
            updater_id,
            time_stamp,
        })),
    )
    .await
    {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };
    match res {
        TableResponse::Done => Ok(upgrade(&app_state, &window).await),
        TableResponse::Err(err) => Err(err.into()),
        _ => unreachable!(),
    }
}

#[tauri::command]
pub async fn upgrade_shift_note(
    app_state: tauri::State<'_, AppState>,
    window: Window,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    note: Note,
    old_content: String,
) -> Result<(), String> {
    if old_content == note.content {
        return Ok(());
    }
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
       return Err("null time stamp".into());
   };
    let Some((updater_id, _)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let res = match main_entry(
        &app_state,
        TableRequest::DepartmentShift(TableCrud::Update(Environment {
            target: UpdateDepartmentShift::UpdateNote(note),
            updater_id,
            time_stamp,
        })),
    )
    .await
    {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };
    match res {
        TableResponse::Done => Ok(upgrade(&app_state, &window).await),
        TableResponse::Err(err) => Err(err.into()),
        _ => unreachable!(),
    }
}

#[tauri::command]
pub async fn fetch_shift_notes_ids(
    app_state: tauri::State<'_, AppState>,
    shift_id: Uuid,
) -> Result<Vec<Uuid>, String> {
    match fetch_shift_notes_ids_by_shift_id(&app_state.pool, shift_id).await {
        Some(ids) => Ok(ids),
        None => Err("".to_string()),
    }
}

#[tauri::command]
pub async fn fetch_shift_note(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<ShiftNote, String> {
    match fetch_shift_note_by_id(&app_state.pool, id).await {
        Some(note) => Ok(note),
        None => Err("".to_string()),
    }
}

#[tauri::command]
pub async fn remove_shift_note(
    app_state: tauri::State<'_, AppState>,
    window: Window,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    note_id: Uuid,
) -> Result<(), String> {
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
       return Err("null time stamp".into());
   };
    let Some((updater_id, shift_id)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let res = match main_entry(
        &app_state,
        TableRequest::DepartmentShift(TableCrud::Update(Environment {
            target: UpdateDepartmentShift::DeleteNote(shift_id, note_id),
            updater_id,
            time_stamp,
        })),
    )
    .await
    {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };
    match res {
        TableResponse::Done => Ok(upgrade(&app_state, &window).await),
        TableResponse::Err(err) => Err(err.into()),
        _ => unreachable!(),
    }
}

#[tauri::command]
pub async fn remove_shift_problem(
    app_state: tauri::State<'_, AppState>,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    window: Window,
    problem_id: Uuid,
) -> Result<(), String> {
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
       return Err("null time stamp".into());
   };
    let Some((updater_id, _)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    let res = match main_entry(
        &app_state,
        TableRequest::ShiftProblem(TableCrud::Delete(
            Environment {
                updater_id,
                target: problem_id,
                time_stamp,
            },
            None,
        )),
    )
    .await
    {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };
    match res {
        TableResponse::Done => Ok(upgrade(&app_state, &window).await),
        TableResponse::Err(err) => Err(err.into()),
        _ => unreachable!(),
    }
}
