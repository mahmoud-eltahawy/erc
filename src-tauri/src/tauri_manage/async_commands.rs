use std::{error::Error, sync::Mutex};

use bcrypt::BcryptResult;
use chrono::{Local, NaiveDateTime, NaiveTime};
use errc::{
    api::main_entry,
    config::AppState,
    memory::{
        department::find_department_name_by_id,
        employee::{does_employee_exist, find_employee_by_card, find_employee_by_id},
        machine::find_machine_name_by_id,
        note::fetch_shift_problem_note,
        problem::find_problem_name_by_id,
        relations::shift_problems::{
            fetch_shift_problem_problems_ids, fetch_shift_problem_spare_parts_ids,
        },
        shift::find_current_department_shift_by_id,
        shift_problem::{find_shift_problem_by_id, find_shift_shift_problems_ids},
        spare_part::find_spare_part_name_by_id,
    },
    syncing::{continious_upgrade, upgrade},
};

use rec::model::{
    employee::Employee,
    note::Note,
    problem::Problem,
    shift::{DepartmentShift, UpdateDepartmentShift},
    shift_problem::{MinimamlShiftProblem, ProblemDetail, ShiftProblem, UpdateShiftProblem},
    Environment, TableCrud, TableRequest, TableResponse,
};
use tauri::Window;
use uuid::Uuid;

fn verify_password(password: String, hash: &str) -> BcryptResult<bool> {
    bcrypt::verify(password, hash)
}

async fn get_or_save_shift_id(
    app_state: &AppState,
    updater_id: &Uuid,
    department_id: &Uuid,
    window: &Window,
) -> Result<Uuid, Box<dyn Error>> {
    if let Ok(id) = find_current_department_shift_by_id(&app_state.pool, department_id).await {
        return Ok(id);
    }

    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
        return Err("null time stamp".into());
    };

    main_entry(
        app_state,
        TableRequest::DepartmentShift(TableCrud::Create(Environment {
            updater_id: *updater_id,
            time_stamp,
            target: DepartmentShift {
                department_id: *department_id,
                id: Uuid::nil(),
                shift_id: Uuid::nil(),
            },
        })),
    )
    .await?;

    upgrade(&app_state, window).await;

    let id = find_current_department_shift_by_id(&app_state.pool, department_id).await?;

    Ok(id)
}

async fn helper(
    app_state: &AppState,
    card_id: i64,
    password: String,
    window: &Window,
) -> Result<(Uuid, Uuid), Box<dyn Error>> {
    let employee = find_employee_by_card(&app_state.pool, card_id).await?;

    let updater_id = Uuid::nil();

    let verified = match verify_password(password, &employee.password) {
        Ok(result) => result,
        Err(err) => return Err(err.into()),
    };

    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
        return Err("null time stamp".into());
    };

    if verified {
        let shift_id =
            get_or_save_shift_id(&app_state, &updater_id, &employee.department_id, window).await?;
        let is_there = does_employee_exist(&app_state.pool, &shift_id, &employee.id).await?;
        if !is_there {
            main_entry(
                app_state,
                TableRequest::DepartmentShift(TableCrud::Update(Environment {
                    updater_id,
                    time_stamp,
                    target: UpdateDepartmentShift::SaveShiftEmployee(shift_id, employee.id),
                })),
            )
            .await?;
        }
        return Ok((employee.id, shift_id));
    }
    Err("".into())
}

#[tauri::command]
pub async fn login(
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    app_state: tauri::State<'_, AppState>,
    window: Window,
    card_id: i64,
    password: String,
) -> Result<(), String> {
    match helper(&app_state, card_id, password, &window).await {
        Ok(result) => {
            let _ = window.emit("new_login", result.0.clone());
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
    if let Err(err) = window.maximize() {
        println!("{err:#?}")
    }
    if !*is_invoked.lock().unwrap() {
        *is_invoked.lock().unwrap() = true;
        continious_upgrade(app_state.inner().clone(), window).await;
    }
    Ok(())
}

#[tauri::command]
pub async fn check_shift_time(
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    app_state: tauri::State<'_, AppState>,
    window: Window,
    department_id: Uuid,
) -> Result<(), String> {
    let failure = Err("فشلت عملية تسجيل الدخول".to_string());

    let Some((employee_id, shift_id)) = *emp_and_uuid.lock().unwrap() else {
        return failure;
    };

    let Ok(new_shift_id) = get_or_save_shift_id(&app_state, &employee_id, &department_id, &window).await else{
        return failure;
    };

    if shift_id == new_shift_id {
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
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    window: Window,
    department_id: Uuid,
    title: String,
    description: String,
) -> Result<(), String> {
    let id = Uuid::new_v4();
    let problem = Problem {
        id,
        department_id,
        title: title.trim().to_string(),
        description,
    };

    let Some((updater_id, _)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };

    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
        return Err("null time stamp".into());
    };

    let Ok(TableResponse::Done) = main_entry(&*app_state,
               TableRequest::Problem(
                   TableCrud::Create(Environment {
                       updater_id,
                       time_stamp,
                       target: problem
                   }))).await else{
        return Err("define problem failed".into());
    };
    upgrade(&app_state, &window).await;
    Ok(())
}

async fn save_minimal_shift_problem(
    app_state: &AppState,
    window: &Window,
    minimal_shift_problem: MinimamlShiftProblem,
    updater_id: Uuid,
) -> Result<(), Box<dyn Error>> {
    let (shift_problem, problems, parts, note) = minimal_shift_problem.destruct();

    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else{
        return Err("null time stamp".into());
    };
    let shift_problem_id = shift_problem.id.clone();

    main_entry(
        app_state,
        TableRequest::ShiftProblem(TableCrud::Create(Environment {
            target: shift_problem,
            updater_id,
            time_stamp,
        })),
    )
    .await?;

    for problem_id in &problems {
        main_entry(
            app_state,
            TableRequest::ShiftProblem(TableCrud::Update(Environment {
                target: UpdateShiftProblem::AddProblem(shift_problem_id, *problem_id),
                updater_id,
                time_stamp,
            })),
        )
        .await?;
    }

    if let Some(parts_ids) = &parts {
        for spare_part_id in parts_ids {
            main_entry(
                app_state,
                TableRequest::ShiftProblem(TableCrud::Update(Environment {
                    target: UpdateShiftProblem::AddSparePart(shift_problem_id, *spare_part_id),
                    updater_id,
                    time_stamp,
                })),
            )
            .await?;
        }
    }

    if let Some(note) = note {
        main_entry(
            app_state,
            TableRequest::ShiftProblem(TableCrud::Update(Environment {
                target: UpdateShiftProblem::AddNote(note),
                updater_id,
                time_stamp,
            })),
        )
        .await?;
    }

    upgrade(app_state, window).await;
    Ok(())
}

async fn update_minimal_shift_problem(
    app_state: &AppState,
    window: Window,
    updater_id: Uuid,
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
    let Some(time_stamp) = NaiveDateTime::from_timestamp_millis(Local::now().timestamp_millis()) else {
        return Err("null time stamp".into());
    };

    if old_maintainer != new_maintainer {
        main_entry(
            app_state,
            TableRequest::ShiftProblem(TableCrud::Update(Environment {
                target: UpdateShiftProblem::UpdateMaintainer(shift_problem_id, new_maintainer),
                updater_id,
                time_stamp,
            })),
        )
        .await?;
    }

    if old_machine != new_machine {
        main_entry(
            app_state,
            TableRequest::ShiftProblem(TableCrud::Update(Environment {
                target: UpdateShiftProblem::UpdateMachine(shift_problem_id, new_machine),
                updater_id,
                time_stamp,
            })),
        )
        .await?;
    }

    if old_begin != new_begin {
        main_entry(
            app_state,
            TableRequest::ShiftProblem(TableCrud::Update(Environment {
                target: UpdateShiftProblem::UpdateBeginTime(shift_problem_id, new_begin),
                updater_id,
                time_stamp,
            })),
        )
        .await?;
    }

    if old_end != new_end {
        main_entry(
            app_state,
            TableRequest::ShiftProblem(TableCrud::Update(Environment {
                target: UpdateShiftProblem::UpdateEndTime(shift_problem_id, new_end),
                updater_id,
                time_stamp,
            })),
        )
        .await?;
    }

    match (old_note, new_note) {
        (Some(old), Some(new)) if old != new => {
            main_entry(
                app_state,
                TableRequest::ShiftProblem(TableCrud::Update(Environment {
                    target: UpdateShiftProblem::UpdateNote(Note {
                        id: shift_problem_id,
                        content: new,
                    }),
                    updater_id,
                    time_stamp,
                })),
            )
            .await?
        }
        (Some(_old), None) => {
            main_entry(
                app_state,
                TableRequest::ShiftProblem(TableCrud::Update(Environment {
                    target: UpdateShiftProblem::DeleteNote(shift_problem_id),
                    updater_id,
                    time_stamp,
                })),
            )
            .await?
        }
        (None, Some(new)) => {
            main_entry(
                app_state,
                TableRequest::ShiftProblem(TableCrud::Update(Environment {
                    target: UpdateShiftProblem::AddNote(Note {
                        id: shift_problem_id,
                        content: new,
                    }),
                    updater_id,
                    time_stamp,
                })),
            )
            .await?
        }
        _ => TableResponse::Done,
    };

    {
        let to_save: Vec<&Uuid> = new_problems
            .iter()
            .filter(|p| !old_problems.contains(p))
            .collect();
        for problem_id in to_save {
            main_entry(
                app_state,
                TableRequest::ShiftProblem(TableCrud::Update(Environment {
                    target: UpdateShiftProblem::AddProblem(shift_problem_id, *problem_id),
                    updater_id,
                    time_stamp,
                })),
            )
            .await?;
        }
    }

    {
        let to_remove: Vec<Uuid> = old_problems
            .into_iter()
            .filter(|p| !new_problems.contains(p))
            .collect();
        for problem_id in to_remove {
            main_entry(
                app_state,
                TableRequest::ShiftProblem(TableCrud::Update(Environment {
                    target: UpdateShiftProblem::DeleteProblem(shift_problem_id, problem_id),
                    updater_id,
                    time_stamp,
                })),
            )
            .await?;
        }
    }

    {
        let to_save: Vec<&Uuid> = new_spare_parts
            .iter()
            .filter(|p| !old_spare_parts.contains(p))
            .collect();
        for part_id in to_save {
            main_entry(
                app_state,
                TableRequest::ShiftProblem(TableCrud::Update(Environment {
                    target: UpdateShiftProblem::AddSparePart(shift_problem_id, *part_id),
                    updater_id,
                    time_stamp,
                })),
            )
            .await?;
        }
    }

    {
        let to_remove: Vec<Uuid> = old_spare_parts
            .into_iter()
            .filter(|p| !new_spare_parts.contains(p))
            .collect();
        for part_id in to_remove {
            main_entry(
                app_state,
                TableRequest::ShiftProblem(TableCrud::Update(Environment {
                    target: UpdateShiftProblem::DeleteSparePart(shift_problem_id, part_id),
                    updater_id,
                    time_stamp,
                })),
            )
            .await?;
        }
    }

    upgrade(app_state, &window).await;
    Ok(())
}

#[tauri::command]
pub async fn save_problem_detail(
    app_state: tauri::State<'_, AppState>,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
    window: Window,
    problem_detail: ProblemDetail,
) -> Result<(), String> {
    let shift_problem = MinimamlShiftProblem::new(problem_detail);
    let Some((updater_id, _)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    match save_minimal_shift_problem(&app_state, &window, shift_problem, updater_id).await {
        Ok(_) => Ok(()),
        Err(err) => return Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn update_problem_detail(
    app_state: tauri::State<'_, AppState>,
    emp_and_uuid: tauri::State<'_, Mutex<Option<(Uuid, Uuid)>>>,
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
    let Some((updater_id, _)) = *emp_and_uuid.lock().unwrap() else {
        return Err("null empoyee id".to_string());
    };
    match update_minimal_shift_problem(
        &app_state,
        window,
        updater_id,
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
) -> Result<String, String> {
    match fetch_shift_problem_note(&app_state.pool, &id).await {
        Ok(x) => Ok(x),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn get_shift_problem_problems_ids_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Vec<Uuid>, String> {
    match fetch_shift_problem_problems_ids(&app_state.pool, &id).await {
        Ok(problems) => Ok(problems),
        Err(_) => Err("empty".to_string()),
    }
}

#[tauri::command]
pub async fn get_shift_problem_spare_parts_ids_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Vec<Uuid>, String> {
    match fetch_shift_problem_spare_parts_ids(&app_state.pool, &id).await {
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

#[tauri::command]
pub async fn get_shift_problem_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<ShiftProblem, String> {
    match find_shift_problem_by_id(&app_state.pool, id).await {
        Ok(problem) => Ok(problem),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn get_machine_name_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<String, String> {
    match find_machine_name_by_id(&app_state.pool, id).await {
        Ok(name) => Ok(name),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn get_department_name_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<String, String> {
    match find_department_name_by_id(&app_state.pool, id).await {
        Ok(name) => Ok(name),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn get_spare_part_name_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<String, String> {
    match find_spare_part_name_by_id(&app_state.pool, id).await {
        Ok(name) => Ok(name),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn get_problem_name_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<String, String> {
    match find_problem_name_by_id(&app_state.pool, id).await {
        Ok(problem) => Ok(problem),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn get_employee_by_id(
    app_state: tauri::State<'_, AppState>,
    id: Uuid,
) -> Result<Employee, String> {
    match find_employee_by_id(&app_state.pool, &id).await {
        Ok(e) => Ok(e),
        Err(err) => Err(err.to_string()),
    }
}
