use rec::{
    crud_sync::Cd,
    model::{TableCrud, TableRequest, TableResponse},
};
use tauri::Window;
use uuid::Uuid;

use crate::{api, config::AppState, memory::shift_problem::find_shift_problem_shift_id};

use std::error::Error;

use super::memory::*;

pub async fn cd_shift_problem(
    app_state: &AppState,
    cud: Cd,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cd::Create => {
            let TableResponse::ShiftProblem(sp) = api::main_entry(
                app_state,
                TableRequest::ShiftProblem(TableCrud::Read(target_id))
            ).await? else { return Err("shift problem does not exist".into()); };
            shift_problem::save(&app_state.pool, &sp).await?;
            window.emit("create_shift_problem", (sp.shift_id, target_id))?;
        }
        Cd::Delete => {
            let shift_id = find_shift_problem_shift_id(&app_state.pool, &target_id).await?;
            shift_problem::delete(&app_state.pool, &target_id).await?;
            window.emit("delete_shift_problem", (shift_id, target_id))?;
        }
    }

    Ok(())
}

pub async fn cd_employee(
    app_state: &AppState,
    cud: Cd,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cd::Create => {
            let TableResponse::Employee(employee) = api::main_entry(
                app_state,
                TableRequest::Employee(TableCrud::Read(target_id))
            ).await? else { return Err("employee does not exist".into()); };
            employee::save(&app_state.pool, employee).await?;
            window.emit("create_employee", target_id)?;
        }
        Cd::Delete => {
            employee::delete(&app_state.pool, target_id).await?;
            window.emit("delete_employee", target_id)?;
        }
    }

    Ok(())
}

pub async fn cd_problem(
    app_state: &AppState,
    cud: Cd,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cd::Create => {
            let TableResponse::Problem(problem) = api::main_entry(
                app_state,
                TableRequest::Problem(TableCrud::Read(target_id))
            ).await? else { return Err("problem does not exist".into()); };
            problem::save(&app_state.pool, problem).await?;
            window.emit("create_problem", target_id)?;
        }
        Cd::Delete => {
            problem::delete(&app_state.pool, target_id).await?;
            window.emit("delete_problem", target_id)?;
        }
    }

    Ok(())
}

pub async fn cd_spare_part(
    app_state: &AppState,
    cud: Cd,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cd::Create => {
            let TableResponse::SparePart(part) = api::main_entry(
                app_state,
                TableRequest::SparePart(TableCrud::Read(target_id))
            ).await? else { return Err("spare part does not exist".into()); };
            spare_part::save(&app_state.pool, part).await?;
            window.emit("create_spare_part", target_id)?;
        }
        Cd::Delete => {
            spare_part::delete(&app_state.pool, target_id).await?;
            window.emit("delete_spare_part", target_id)?;
        }
    }

    Ok(())
}

pub async fn cd_machine(
    app_state: &AppState,
    cud: Cd,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cd::Create => {
            let TableResponse::Machine(mac) = api::main_entry(
                app_state,
                TableRequest::Machine(TableCrud::Read(target_id))
            ).await? else { return Err("machine does not exist".into()); };
            machine::save(&app_state.pool, mac).await?;
            window.emit("create_machine", target_id)?;
        }
        Cd::Delete => {
            machine::delete(&app_state.pool, target_id).await?;
            window.emit("delete_machine", target_id)?;
        }
    }

    Ok(())
}

pub async fn cd_department(
    app_state: &AppState,
    cud: Cd,
    target_id: Uuid,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cd::Create => {
            let TableResponse::Department(dep) = api::main_entry(
                app_state,
                TableRequest::Department(TableCrud::Read(target_id))
            ).await? else { return Err("department does not exist".into()); };
            department::save(&app_state.pool, dep).await?;
            window.emit("create_department", target_id)?;
        }
        Cd::Delete => {
            department::delete(&app_state.pool, target_id).await?;
            window.emit("delete_department", target_id)?;
        }
    }

    Ok(())
}

pub async fn cd_shift(
    app_state: &AppState,
    cud: Cd,
    target_id: Uuid,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cd::Create => {
            let shift = api::fetch_shift(app_state, &target_id).await?;
            shift::save(&app_state.pool, shift).await?
        }
        Cd::Delete => return Err("note crud implemented in note section".into()),
    }
    Ok(())
}

pub async fn cd_department_shift(
    app_state: &AppState,
    cud: Cd,
    target_id: Uuid,
) -> Result<(), Box<dyn Error>> {
    match cud {
        Cd::Create => {
            let TableResponse::DepartmentShift(ds) = api::main_entry(
                app_state,
                TableRequest::DepartmentShift(TableCrud::Read(target_id))
            ).await? else { return Err("department does not exist".into()); };
            shift::save_department_shift(&app_state.pool, ds).await?;
        }
        Cd::Delete => shift::delete_department_shift(&app_state.pool, target_id).await?,
    }
    Ok(())
}
