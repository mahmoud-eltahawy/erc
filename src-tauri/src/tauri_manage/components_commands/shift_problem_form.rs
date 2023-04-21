use chrono::NaiveTime;
use errc::{
    api::{
        note::save_note_to_shift,
        shift::{delete_shift_employee, save_shift_employee},
        shift_problem::delete_shift_problem,
    },
    config::AppState,
    memory::{
        employee::{
            find_4_employees, find_employees_by_name, find_shift_existing_employees_names,
            find_shift_non_existing_employees_names,
        },
        machine::{find_4_machines, find_machines_by_name},
        note::fetch_notes_content_by_shift_id,
        problem::{find_department_4_problems, find_department_problems_by_name},
        spare_part::{find_4_spare_parts, find_spare_parts_by_name},
    },
};
use rec::{
    model::{name::Name, note::ShiftNote},
    timer::{get_current_order, get_relative_now, get_shift_borders},
};
use uuid::Uuid;

#[tauri::command]
pub async fn problems_selection(
    app_state: tauri::State<'_, AppState>,
    department_id: Uuid,
    name: Option<String>,
    canceled: Vec<String>,
) -> Result<Vec<Name<String>>, String> {
    if let Some(name) = name {
        match find_department_problems_by_name(
            &app_state.pool,
            department_id.to_string(),
            &name.trim(),
            canceled,
        )
        .await
        {
            Ok(p) => Ok(p),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_department_4_problems(&app_state.pool, department_id.to_string(), canceled).await
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
    canceled: Vec<String>,
) -> Result<Vec<Name<String>>, String> {
    if let Some(name) = name {
        match find_employees_by_name(&app_state.pool, &name.trim(), canceled).await {
            Ok(e) => Ok(e),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_4_employees(&app_state.pool, canceled).await {
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
) -> Result<Vec<Name<String>>, String> {
    if let Some(name) = name {
        match find_machines_by_name(&app_state.pool, &name.trim(), canceled).await {
            Ok(e) => Ok(e),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_4_machines(&app_state.pool, canceled).await {
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
) -> Result<Vec<Name<String>>, String> {
    if let Some(name) = name {
        match find_spare_parts_by_name(&app_state.pool, &name.trim(), canceled).await {
            Ok(e) => Ok(e),
            Err(err) => Err(err.to_string()),
        }
    } else {
        match find_4_spare_parts(&app_state.pool, canceled).await {
            Ok(e) => Ok(e),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[tauri::command]
pub async fn shift_existing_employees(
    app_state: tauri::State<'_, AppState>,
    shift_id: Uuid,
) -> Result<Vec<Name<String>>, String> {
    match find_shift_existing_employees_names(&app_state.pool, shift_id).await {
        Ok(e) => Ok(e),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn shift_non_existing_employees(
    app_state: tauri::State<'_, AppState>,
    shift_id: Uuid,
    department_id: Uuid,
) -> Result<Vec<Name<String>>, String> {
    match find_shift_non_existing_employees_names(&app_state.pool, shift_id, department_id).await {
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
    shift_id: Uuid,
    employee_id: Uuid,
) -> Result<(), String> {
    match save_shift_employee(&app_state, &shift_id, &employee_id).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn remove_shift_employee(
    app_state: tauri::State<'_, AppState>,
    shift_id: Uuid,
    employee_id: Uuid,
) -> Result<(), String> {
    match delete_shift_employee(&app_state, &shift_id, &employee_id).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn save_shift_note(
    app_state: tauri::State<'_, AppState>,
    shift_id: Uuid,
    writer_id: Uuid,
    content: String,
) -> Result<(), String> {
    match save_note_to_shift(
        &app_state,
        &ShiftNote {
            id: Uuid::new_v4(),
            shift_id,
            writer_id,
            content,
        },
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn fetch_shift_notes(
    app_state: tauri::State<'_, AppState>,
    shift_id: Uuid,
) -> Result<Vec<(Uuid, String)>, String> {
    let shift_id = shift_id.to_string();
    match fetch_notes_content_by_shift_id(&app_state.pool, shift_id).await {
        Some(ids_notes) => Ok(ids_notes),
        None => Err("".to_string()),
    }
}

#[tauri::command]
pub async fn remove_shift_problem(
    app_state: tauri::State<'_, AppState>,
    problem_id: Uuid,
) -> Result<(), String> {
    match delete_shift_problem(&app_state, &problem_id).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
