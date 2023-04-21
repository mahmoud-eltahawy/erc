mod api;
mod memory;

use crate::{config::AppState, memory::shift_problem::find_shift_problem_shift_id};
use std::error::Error;

use itertools::Itertools;
use rec::{
    crud_sync::{Cud, CudVersion, Table},
    model::note::Note,
};
use tauri::Window;
use uuid::Uuid;

use memory::{
    department, employee, machine, note, problem, relations, shift, shift_problem, spare_part,
    syncing,
};

use self::memory::permissions;

pub async fn continious_upgrade(app_state: AppState, window: Window) -> Result<(), Box<dyn Error>> {
    loop {
        upgrade(&app_state, &window).await?
    }
}

pub async fn upgrade(app_state: &AppState, window: &Window) -> Result<(), Box<dyn Error>> {
    let version = syncing::last_version(&app_state.pool).await?;
    let updates = api::updates(app_state, version as u64).await?;
    let mut errors = Vec::new();
    for update in updates {
        if let Err(err) = apply_update(app_state, update, window).await {
            errors.push(err.to_string());
        }
    }
    if !errors.is_empty() {
        return Err(errors.into_iter().join(" \n ").into());
    }
    Ok(())
}

async fn apply_update(
    app_state: &AppState,
    cud_version: CudVersion,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    let CudVersion {
        version_number,
        cud,
        target_id,
        target_table,
        other_target_id,
    } = cud_version;
    match target_table {
        Table::Employee => update_employee(app_state, cud, target_id, window).await?,
        Table::Problem => update_problem(app_state, cud, target_id, window).await?,
        Table::SparePart => update_spare_part(app_state, cud, target_id, window).await?,
        Table::Machine => update_machine(app_state, cud, target_id, window).await?,
        Table::ShiftProblem => update_shift_problem(app_state, cud, target_id, window).await?,
        Table::Shift => update_shift(app_state, cud, target_id).await?,
        Table::Department => update_department(app_state, cud, target_id, window).await?,
        Table::ShiftProblemNote => {
            update_shift_problem_note(app_state, cud, target_id, window).await?
        }
        Table::ShiftNote => update_shift_note(app_state, cud, other_target_id, window).await?,
        Table::DepartmentShift => update_department_shift(app_state, cud, target_id).await?,
        Table::ShiftProblemProblem => {
            update_shift_problem_problem(app_state, cud, target_id, other_target_id, window).await?
        }
        Table::ShiftProblemSparePart => {
            update_shift_problem_spare_part(app_state, cud, target_id, other_target_id, window)
                .await?
        }
        Table::Permissions => update_permissions(app_state, cud, target_id, window).await?,
        Table::DepartmentShiftEmployee => {
            update_department_shift_employee(app_state, cud, target_id, other_target_id, window)
                .await?
        }
    }
    syncing::save_version(&app_state.pool, version_number).await?;
    Ok(())
}

async fn update_department_shift_employee(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
    other_id: Option<Uuid>,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => match other_id {
            Some(id) => {
                relations::shift_employee::save(&app_state.pool, target_id, id).await?;
            }
            None => return Err("the shift employee id is null".into()),
        },
        Cud::Delete => match other_id {
            Some(id) => relations::shift_employee::delete(&app_state.pool, target_id, id).await?,
            None => return Err("the shift employee id is null".into()),
        },
        Cud::Update => return Err("shift employee is only created or deleted".into()),
    }

    window.emit("update_department_shift_employee", target_id)?;

    Ok(())
}

async fn update_permissions(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => {
            let per = api::permissions(app_state, target_id).await?;
            permissions::save(&app_state.pool, per).await?;
        }
        Cud::Update => {
            let per = api::permissions(app_state, target_id).await?;
            permissions::update(&app_state.pool, per).await?;
        }
        Cud::Delete => return Err("permissions can not be deleted".into()),
    }

    window.emit("update_permissions", target_id)?;

    Ok(())
}

async fn update_shift_problem(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => {
            let sp = api::shift_problem(app_state, target_id).await?;
            shift_problem::save(&app_state.pool, &sp).await?;
            window.emit("create_shift_problem", (sp.shift_id, target_id))?;
        }
        Cud::Delete => {
            let shift_id = find_shift_problem_shift_id(&app_state.pool, &target_id).await?;
            shift_problem::delete(&app_state.pool, &target_id).await?;
            window.emit("delete_shift_problem", (shift_id, target_id))?;
        }
        Cud::Update => {
            let sp = api::shift_problem(app_state, target_id).await?;
            shift_problem::update(&app_state.pool, &sp).await?;
            window.emit("update_shift_problem", (sp.shift_id, target_id))?;
        }
    }

    Ok(())
}

async fn update_employee(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => {
            let employee = api::employee(app_state, target_id).await?;
            employee::save(&app_state.pool, employee).await?;
        }
        Cud::Delete => employee::delete(&app_state.pool, target_id).await?,
        Cud::Update => {
            let employee = api::employee(app_state, target_id).await?;
            employee::update(&app_state.pool, employee).await?
        }
    }

    window.emit("update_employee", "hello")?;

    Ok(())
}

async fn update_problem(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => {
            let problem = api::problem(app_state, target_id).await?;
            problem::save(&app_state.pool, problem).await?;
        }
        Cud::Delete => problem::delete(&app_state.pool, target_id).await?,
        Cud::Update => {
            let problem = api::problem(app_state, target_id).await?;
            problem::update(&app_state.pool, problem).await?;
        }
    }

    window.emit("update_problem", "hello")?;

    Ok(())
}

async fn update_spare_part(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => {
            let part = api::spare_part(app_state, target_id).await?;
            spare_part::save(&app_state.pool, part).await?;
        }
        Cud::Delete => spare_part::delete(&app_state.pool, target_id).await?,
        Cud::Update => {
            let part = api::spare_part(app_state, target_id).await?;
            spare_part::update(&app_state.pool, part).await?;
        }
    }

    window.emit("update_spare_part", "hello")?;

    Ok(())
}

async fn update_machine(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => {
            let mac = api::machine(app_state, target_id).await?;
            machine::save(&app_state.pool, mac).await?;
        }
        Cud::Delete => machine::delete(&app_state.pool, target_id).await?,
        Cud::Update => {
            let mac = api::machine(app_state, target_id).await?;
            machine::update(&app_state.pool, mac).await?;
        }
    }

    window.emit("update_machine", "hello")?;

    Ok(())
}

async fn update_department(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => {
            let dep = api::department(app_state, target_id).await?;
            department::save(&app_state.pool, dep).await?;
        }
        Cud::Delete => department::delete(&app_state.pool, target_id).await?,
        Cud::Update => {
            let dep = api::department(app_state, target_id).await?;
            department::update(&app_state.pool, dep).await?;
        }
    }

    window.emit("update_departments", target_id)?;

    Ok(())
}

async fn update_shift_note(
    app_state: &AppState,
    cud: Cud,
    target_id: Option<Uuid>,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    let Some(target_id) = target_id else {
        return Err("null id".into());
    };
    match cud {
        Cud::Create => {
            let note = api::shift_note(app_state, target_id).await?;
            note::save_to_shift(&app_state.pool, note).await?;
        }
        Cud::Delete => note::delete_shift_note(&app_state.pool, target_id).await?,
        Cud::Update => {
            let note = api::shift_note(app_state, target_id).await?;
            note::update_shift_note(
                &app_state.pool,
                Note {
                    id: note.id,
                    content: note.content,
                },
            )
            .await?;
        }
    }

    window.emit("update_shift_note", target_id)?;

    Ok(())
}

async fn update_shift_problem_note(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => {
            let note = api::shift_problem_note(app_state, target_id).await?;
            note::save_to_shift_problem(&app_state.pool, note).await?;
        }
        Cud::Update => {
            let note = api::shift_problem_note(app_state, target_id).await?;
            note::update_shift_problem_note(&app_state.pool, note).await?;
        }
        Cud::Delete => note::delete_shift_problem_note(&app_state.pool, target_id).await?,
    }

    window.emit("update_shift_problem_note", target_id)?;

    Ok(())
}

async fn update_shift(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => {
            let shift = api::shift(app_state, target_id).await?;
            shift::save(&app_state.pool, shift).await?
        }
        Cud::Update | Cud::Delete => return Err("note crud implemented in note section".into()),
    }
    Ok(())
}

async fn update_department_shift(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => {
            let ds = api::shift_department(app_state, target_id).await?;
            shift::save_department_shift(&app_state.pool, ds).await?;
        }
        Cud::Delete => shift::delete_department_shift(&app_state.pool, target_id).await?,
        Cud::Update => return Err("shift is only created or deleted table".into()),
    }
    Ok(())
}

async fn update_shift_problem_problem(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
    other_id: Option<Uuid>,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => match other_id {
            Some(id) => {
                relations::shift_problems::save_problem(&app_state.pool, id, target_id).await?;
            }
            None => return Err("the shift problem id is null".into()),
        },
        Cud::Delete => match other_id {
            Some(id) => {
                relations::shift_problems::delete_problem(&app_state.pool, id, target_id).await?;
            }
            None => return Err("the shift problem id is null".into()),
        },
        Cud::Update => return Err("shift is only created or deleted table".into()),
    }
    window.emit("update_shift_problem_problem", target_id)?;
    Ok(())
}

async fn update_shift_problem_spare_part(
    app_state: &AppState,
    cud: Cud,
    target_id: Uuid,
    other_id: Option<Uuid>,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cud::Create => match other_id {
            Some(id) => {
                relations::shift_problems::save_spare_part(&app_state.pool, id, target_id).await?;
            }
            None => return Err("the shift problem id is null".into()),
        },
        Cud::Delete => match other_id {
            Some(id) => {
                relations::shift_problems::delete_spare_part(&app_state.pool, id, target_id).await?
            }
            None => return Err("the shift problem id is null".into()),
        },
        Cud::Update => return Err("shift is only created or deleted table".into()),
    }
    window.emit("update_shift_problem_parts", target_id)?;
    Ok(())
}
