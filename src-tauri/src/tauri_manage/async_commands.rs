use std::{error::Error, str::FromStr, sync::Mutex};

use bcrypt::BcryptResult;
use errc::{
    api::{
        note::save_note_to_problem,
        problem::save_problem,
        relations::shift_problem::{
            save_problem_to_shift_problem, save_spare_part_to_shift_problem,
        },
        shift::{save_shift, save_shift_employee},
        shift_problem::save_shift_problem,
    },
    config::AppState,
    memory::{
        employee::{does_employee_exist, find_employee_by_card, find_employee_by_id},
        machine::find_machine_by_id,
        note::fetch_shift_problem_note,
        problem::find_problem_by_id,
        relations::shift_problems::{
            fetch_shift_problem_problems, fetch_shift_problem_spare_parts,
        },
        shift::find_current_department_shift_by_id,
        shift_problem::find_shift_shift_problems,
        spare_part::find_spare_part_by_id,
    },
    syncing::{continious_upgrade, upgrade},
};
use rec::model::{
    employee::Employee,
    machine::Machine,
    note::{DbNote, Note},
    problem::Problem,
    shift_problem::{ClientMinimamlShiftProblem, MinimamlShiftProblem, ProblemDetail},
    spare_part::SparePart,
};
use tauri::Window;
use uuid::Uuid;

fn verify_password(password: String, hash: &str) -> BcryptResult<bool> {
    bcrypt::verify(password, hash)
}

async fn get_or_save_shift_id(
    app_state: &AppState,
    department_id: &String,
    window: &Window,
) -> Result<String, Box<dyn Error>> {
    let id_f = find_current_department_shift_by_id(&app_state.pool, department_id);

    if let Ok(id) = id_f.await {
        return Ok(id);
    }

    save_shift(app_state, department_id).await?;

    upgrade(&app_state, Some(window)).await?;

    let id = find_current_department_shift_by_id(&app_state.pool, department_id).await?;

    Ok(id)
}

async fn helper(
    app_state: &AppState,
    card_id: i64,
    password: String,
    window: &Window,
) -> Result<(Employee<String>, String), Box<dyn Error>> {
    let employee = find_employee_by_card(&app_state.pool, card_id).await?;

    let verified = match verify_password(password, &employee.password) {
        Ok(result) => result,
        Err(err) => return Err(err.into()),
    };

    if verified {
        let shift_id = get_or_save_shift_id(&app_state, &employee.department_id, window).await?;
        let shift_uuid = Uuid::from_str(&shift_id)?;
        let employee_id = Uuid::from_str(&employee.id)?;
        let is_there = does_employee_exist(&app_state.pool, &shift_uuid, &employee_id).await?;
        if !is_there {
            save_shift_employee(&app_state, &shift_uuid, &employee_id).await?;
        }
        return Ok((employee, shift_id));
    }
    Err("".into())
}

#[tauri::command]
pub async fn login(
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Employee<String>, String)>>>,
    app_state: tauri::State<'_, AppState>,
    window: Window,
    card_id: i64,
    password: String,
) -> Result<(), String> {
    match helper(&app_state, card_id, password, &window).await {
        Ok(result) => {
            let _ = window.emit("new_login", result.0.id.clone());
            *emp_and_uuid.lock().unwrap() = Some(result);
            Ok(())
        }
        Err(err) => return Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn update(
    is_invoked: tauri::State<'_, Mutex<bool>>,
    app_state: tauri::State<'_, AppState>,
    window: Window,
) -> Result<(), String> {
    if !*is_invoked.lock().unwrap() {
        *is_invoked.lock().unwrap() = true;
        if let Err(err) = continious_upgrade(app_state.inner().clone(), window).await {
            println!("{}", err.to_string());
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn check_shift_time(
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Employee<String>, String)>>>,
    app_state: tauri::State<'_, AppState>,
    window: Window,
    department_id: Uuid,
) -> Result<(), String> {
    let failure = Err("فشلت عملية تسجيل الدخول".to_string());

    let nid = &mut get_or_save_shift_id(&app_state, &department_id.to_string(), &window).await;

    let nid = match nid {
        Ok(v) => v,
        Err(_) => return failure,
    };

    let l = &*emp_and_uuid.lock().unwrap();

    let (_, cid) = match l {
        Some(v) => v,
        None => return failure,
    };

    if cid == nid {
        Ok(())
    } else {
        match window.emit("shift_ended", None::<&str>) {
            Ok(_) => Ok(()),
            Err(_) => failure,
        }
    }
}

#[tauri::command]
pub async fn define_problem(
    app_state: tauri::State<'_, AppState>,
    window: Window,
    writer_id: Uuid,
    department_id: Uuid,
    title: String,
    description: String,
) -> Result<(), String> {
    let id = Uuid::new_v4();
    let problem = Problem {
        id,
        writer_id,
        department_id,
        title: title.trim().to_string(),
        description,
    };
    match save_problem(&app_state, &problem).await {
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),
    };

    match upgrade(&app_state, Some(&window)).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(err.to_string()),
    }
}

async fn save_minimal_shift_problem(
    app_state: &AppState,
    minimal_shift_problem: MinimamlShiftProblem,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    let (shift_problem, problems, parts, note) = minimal_shift_problem.destruct();
    save_shift_problem(app_state, &shift_problem).await?;
    let shift_problem_id = shift_problem.id;
    for problem_id in &problems {
        save_problem_to_shift_problem(app_state, problem_id, &shift_problem_id).await?;
    }

    if let Some(parts_ids) = &parts {
        for spare_part_id in parts_ids {
            save_spare_part_to_shift_problem(app_state, spare_part_id, &shift_problem_id).await?;
        }
    }

    if let Some(note) = note.clone() {
        let Note { id, content } = note;
        let shift_problem_id = Some(shift_problem_id);
        save_note_to_problem(
            app_state,
            &DbNote {
                id,
                content,
                shift_problem_id,
                shift_id: None,
            },
        )
        .await?;
    }

    upgrade(app_state, Some(window)).await?;
    Ok(())
}

#[tauri::command]
pub async fn save_problem_detail(
    problem_detail: ProblemDetail,
    window: Window,
    app_state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let shift_problem = MinimamlShiftProblem::new(problem_detail);
    match save_minimal_shift_problem(&app_state, shift_problem, &window).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(err.to_string()),
    }
}

async fn fetch_minimal_shift_problem_by_shift_id(
    app_state: &AppState,
    shift_id: Uuid,
) -> Result<Vec<ClientMinimamlShiftProblem>, Box<dyn Error>> {
    let shift_problems = find_shift_shift_problems(&app_state.pool, shift_id.to_string()).await?;
    let mut result = Vec::new();
    for sp in shift_problems {
        let problems = fetch_shift_problem_problems(&app_state.pool, &sp.id).await?;
        let parts = fetch_shift_problem_spare_parts(&app_state.pool, &sp.id).await?;
        let parts = if parts.is_empty() { None } else { Some(parts) };
        let note = fetch_shift_problem_note(&app_state.pool, &sp.id).await;
        result.push(ClientMinimamlShiftProblem::construct((
            sp, problems, parts, note,
        )));
    }
    Ok(result)
}

#[tauri::command]
pub async fn get_current_shift_problems(
    app_state: tauri::State<'_, AppState>,
    shift_id: Uuid,
) -> Result<Vec<ClientMinimamlShiftProblem>, String> {
    match fetch_minimal_shift_problem_by_shift_id(&app_state, shift_id).await {
        Ok(problems) => Ok(problems.to_vec()),
        Err(_) => Err("empty".to_string()),
    }
}

#[tauri::command]
pub async fn get_problem_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Problem<String>, String> {
    match find_problem_by_id(&app_state.pool, id.to_string()).await {
        Ok(problem) => Ok(problem),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn get_machine_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Machine<String>, String> {
    match find_machine_by_id(&app_state.pool, id.to_string()).await {
        Ok(mac) => Ok(mac),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn get_spare_part_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<SparePart<String>, String> {
    match find_spare_part_by_id(&app_state.pool, id.to_string()).await {
        Ok(s) => Ok(s),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn get_employee_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Employee<String>, String> {
    match find_employee_by_id(&app_state.pool, id.to_string()).await {
        Ok(e) => Ok(e),
        Err(err) => Err(err.to_string()),
    }
}
