use std::{error::Error, str::FromStr, sync::Mutex};

use bcrypt::BcryptResult;
use chrono::NaiveTime;
use errc::{
    api::{
        note::{delete_problem_note, save_note_to_problem, update_problem_note},
        problem::save_problem,
        relations::shift_problem::{
            delete_problem_from_shift_problem, delete_spare_part_from_shift_problem,
            save_problem_to_shift_problem, save_spare_part_to_shift_problem,
        },
        shift::{save_shift, save_shift_employee},
        shift_problem::{save_shift_problem, update_shift_problem},
    },
    config::AppState,
    memory::{
        employee::{
            does_employee_exist, find_employee_by_card, find_employee_by_id,
            find_employee_name_by_id,
        },
        machine::find_machine_by_id,
        note::fetch_shift_problem_note,
        problem::find_problem_by_id,
        relations::shift_problems::{
            fetch_shift_problem_problems_names, fetch_shift_problem_spare_parts_names,
        },
        shift::find_current_department_shift_by_id,
        shift_problem::{find_shift_problem_by_id, find_shift_shift_problems_ids},
        spare_part::find_spare_part_by_id,
    },
    syncing::{continious_upgrade, upgrade},
};

use rec::model::{
    employee::Employee,
    name::Name,
    note::Note,
    problem::Problem,
    shift_problem::{
        ClientShiftProblem, MinimamlShiftProblem, ProblemDetail, ShiftProblem, ShiftProblemNames,
    },
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

    upgrade(&app_state, window).await?;

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

    match upgrade(&app_state, &window).await {
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

    if let Some(note) = note {
        save_note_to_problem(app_state, &note).await?;
    }

    upgrade(app_state, window).await?;
    Ok(())
}

async fn update_minimal_shift_problem(
    app_state: &AppState,
    window: Window,
    shift_problem_id: Uuid,
    (
        (old_maintainer, new_maintainer),
        (old_machine, new_machine),
        (old_begin, new_begin),
        (old_end, new_end),
    ): (
        (Uuid, Uuid),
        (Uuid, Uuid),
        (NaiveTime, NaiveTime),
        (NaiveTime, NaiveTime),
    ),
    (old_problems, new_problems): (Vec<Uuid>, Vec<Uuid>),
    (old_spare_parts, new_spare_parts): (Vec<Uuid>, Vec<Uuid>),
    (old_note, new_note): (Option<String>, Option<String>),
) -> Result<(), Box<dyn Error>> {
    if old_maintainer != new_maintainer
        || old_machine != new_machine
        || old_begin != new_begin
        || old_end != new_end
    {
        update_shift_problem(
            app_state,
            &ShiftProblem {
                id: shift_problem_id,
                shift_id: Uuid::nil(),  //NOTE can note be updated
                writer_id: Uuid::nil(), //NOTE can note be updated
                maintainer_id: new_maintainer,
                machine_id: new_machine,
                begin_time: new_begin,
                end_time: new_end,
            },
        )
        .await?;
    }

    match (old_note, new_note) {
        (Some(old), Some(new)) => {
            if old != new {
                update_problem_note(
                    app_state,
                    &Note {
                        id: shift_problem_id,
                        content: new,
                    },
                )
                .await?;
            }
        }
        (Some(_old), None) => delete_problem_note(app_state, &shift_problem_id).await?,
        (None, Some(new)) => {
            save_note_to_problem(
                app_state,
                &Note {
                    id: shift_problem_id,
                    content: new,
                },
            )
            .await?
        }
        (None, None) => {}
    }

    {
        let to_save: Vec<&Uuid> = new_problems
            .iter()
            .filter(|p| !old_problems.contains(p))
            .collect();
        for problem_id in to_save {
            save_problem_to_shift_problem(app_state, problem_id, &shift_problem_id).await?;
        }
    }

    {
        let to_remove: Vec<Uuid> = old_problems
            .into_iter()
            .filter(|p| !new_problems.contains(p))
            .collect();
        for problem_id in to_remove {
            delete_problem_from_shift_problem(app_state, &problem_id, &shift_problem_id).await?;
        }
    }

    {
        let to_save: Vec<&Uuid> = new_spare_parts
            .iter()
            .filter(|p| !old_spare_parts.contains(p)).collect();
        for part_id in to_save {
            save_spare_part_to_shift_problem(app_state, part_id, &shift_problem_id).await?;
        }
    }

    {
        let to_remove: Vec<Uuid> = old_spare_parts
            .into_iter()
            .filter(|p| !new_spare_parts.contains(p)).collect();
        for part_id in to_remove {
            delete_spare_part_from_shift_problem(app_state, &part_id, &shift_problem_id)
                .await?;
        }
    }

    upgrade(app_state, &window).await?;
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

#[tauri::command]
pub async fn update_problem_detail(
    app_state: tauri::State<'_, AppState>,
    window: Window,
    shift_problem_id: Uuid,
    core: (
        (Uuid, Uuid),           //maintainer
        (Uuid, Uuid),           // machine
        (NaiveTime, NaiveTime), //  begin
        (NaiveTime, NaiveTime), //   end
    ),
    problems: (Vec<Uuid>, Vec<Uuid>),
    spare_parts: (Vec<Uuid>, Vec<Uuid>),
    note: (Option<String>, Option<String>),
) -> Result<(), String> {
    match update_minimal_shift_problem(
        &app_state,
        window,
        shift_problem_id,
        core,
        problems,
        spare_parts,
        note,
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => return Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn get_shift_problem_note_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Note<String>, String> {
    match fetch_shift_problem_note(&app_state.pool, &id).await {
        Some(x) => Ok(x),
        None => Err("".to_string()),
    }
}

#[tauri::command]
pub async fn get_shift_problem_problems_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Vec<Name<Uuid>>, String> {
    match fetch_shift_problem_problems_names(&app_state.pool, &id).await {
        Ok(problems) => Ok(problems),
        Err(_) => Err("empty".to_string()),
    }
}

#[tauri::command]
pub async fn get_shift_problem_spare_parts_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Vec<Name<Uuid>>, String> {
    match fetch_shift_problem_spare_parts_names(&app_state.pool, &id).await {
        Ok(parts) => Ok(parts),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn get_shift_problems_ids_by_shift_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Vec<Uuid>, String> {
    match find_shift_shift_problems_ids(&app_state.pool, id).await {
        Ok(problems) => Ok(problems),
        Err(_) => Err("empty".to_string()),
    }
}

async fn extract_shift_problem_names(
    app_state: &AppState,
    sp: ClientShiftProblem,
) -> Result<ShiftProblemNames, Box<dyn Error>> {
    let ClientShiftProblem {
        id,
        shift_id,
        writer_id,
        machine_id,
        maintainer_id,
        begin_time,
        end_time,
    } = sp;
    let id = Uuid::from_str(id.as_str())?;
    let shift_id = Uuid::from_str(shift_id.as_str())?;
    let writer_id = Uuid::from_str(writer_id.as_str())?;
    let machine_id = Uuid::from_str(machine_id.as_str())?;
    let maintainer_id = Uuid::from_str(maintainer_id.as_str())?;

    Ok(ShiftProblemNames {
        id,
        shift_id,
        writer: Name {
            id: writer_id,
            name: find_employee_name_by_id(&app_state.pool, writer_id).await?,
        },
        maintainer: Name {
            id: maintainer_id,
            name: find_employee_name_by_id(&app_state.pool, maintainer_id).await?,
        },
        machine: find_machine_by_id(&app_state.pool, machine_id).await?,
        begin_time: serde_json::from_str(&begin_time)?,
        end_time: serde_json::from_str(&end_time)?,
    })
}

#[tauri::command]
pub async fn get_shift_problem_names_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<ShiftProblemNames, String> {
    match find_shift_problem_by_id(&app_state.pool, id).await {
        Ok(problem) => match extract_shift_problem_names(&app_state, problem).await {
            Ok(p) => Ok(p),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
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
) -> Result<Name<Uuid>, String> {
    match find_machine_by_id(&app_state.pool, id).await {
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
