use rec::model::{
    department::UpdateDepartment, employee::UpdateEmployee, machine::UpdateMachine, note::Note,
    problem::UpdateProblem, shift::UpdateDepartmentShift, shift_problem::UpdateShiftProblem,
    spare_part::UpdateSparePart,
};
use tauri::Window;

use std::error::Error;

use crate::config::AppState;

use super::{
    memory::{relations::*, *},
    Env,
};

pub async fn update_shift_problem(
    app_state: &AppState,
    sp: UpdateShiftProblem,
    env: Env,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match sp {
        UpdateShiftProblem::UpdateBeginTime(shift_problem_id, begin_time) => {
            shift_problem::update_begin_time(&app_state.pool, shift_problem_id, begin_time).await?;
            window.emit(
                "update_shift_problem_begin_time",
                (shift_problem_id, begin_time),
            )?
        }
        UpdateShiftProblem::UpdateEndTime(shift_problem_id, end_time) => {
            shift_problem::update_end_time(&app_state.pool, shift_problem_id, end_time).await?;
            window.emit(
                "update_shift_problem_end_time",
                (shift_problem_id, end_time),
            )?
        }
        UpdateShiftProblem::UpdateMachine(shift_problem_id, machine_id) => {
            shift_problem::update_machine(&app_state.pool, shift_problem_id, machine_id).await?;
            window.emit(
                "update_shift_problem_machine",
                (shift_problem_id, machine_id),
            )?
        }
        UpdateShiftProblem::UpdateMaintainer(shift_problem_id, maintainer_id) => {
            shift_problem::update_maintainer(&app_state.pool, shift_problem_id, maintainer_id)
                .await?;
            window.emit(
                "update_shift_problem_maintainer",
                (shift_problem_id, maintainer_id),
            )?
        }
        UpdateShiftProblem::AddProblem(shift_problem_id, problem_id) => {
            relations::shift_problems::save_problem(
                &app_state.pool,
                shift_problem_id,
                problem_id,
                env,
            )
            .await?;
            window.emit(
                "update_shift_problem_add_problem",
                (shift_problem_id, problem_id),
            )?
        }
        UpdateShiftProblem::DeleteProblem(shift_problem_id, problem_id) => {
            relations::shift_problems::delete_problem(
                &app_state.pool,
                shift_problem_id,
                problem_id,
            )
            .await?;
            window.emit(
                "update_shift_problem_delete_problem",
                (shift_problem_id, problem_id),
            )?
        }
        UpdateShiftProblem::AddSparePart(shift_problem_id, part_id) => {
            relations::shift_problems::save_spare_part(
                &app_state.pool,
                shift_problem_id,
                part_id,
                env,
            )
            .await?;
            window.emit(
                "update_shift_problem_add_spare_part",
                (shift_problem_id, part_id),
            )?
        }
        UpdateShiftProblem::DeleteSparePart(shift_problem_id, part_id) => {
            relations::shift_problems::delete_spare_part(
                &app_state.pool,
                shift_problem_id,
                part_id,
            )
            .await?;
            window.emit(
                "update_shift_problem_delete_spare_part",
                (shift_problem_id, part_id),
            )?
        }
        UpdateShiftProblem::AddNote(note) => {
            note::save_to_shift_problem(&app_state.pool, &note, env).await?;
            let Note { id, content } = note;
            window.emit("update_shift_problem_add_note", (id, content))?
        }
        UpdateShiftProblem::DeleteNote(shift_problem_id) => {
            note::delete_shift_problem_note(&app_state.pool, shift_problem_id).await?;
            window.emit("update_shift_problem_delete_note", shift_problem_id)?
        }
        UpdateShiftProblem::UpdateNote(note) => {
            note::update_shift_problem_note(&app_state.pool, &note).await?;
            let Note { id, content } = note;
            window.emit("update_shift_problem_update_note", (id, content))?
        }
    }

    Ok(())
}

pub async fn update_problem(
    app_state: &AppState,
    pro: UpdateProblem,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match pro {
        UpdateProblem::UpdateTitle(problem_id, title) => {
            problem::update_title(&app_state.pool, &problem_id, &title).await?;
            window.emit("update_problem_title", (problem_id, title))?
        }
        UpdateProblem::UpdateDescription(problem_id, description) => {
            problem::update_description(&app_state.pool, &problem_id, &description).await?;
            window.emit("update_problem_description", (problem_id, description))?
        }
    }

    Ok(())
}

pub async fn update_spare_part(
    app_state: &AppState,
    part: UpdateSparePart,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match part {
        UpdateSparePart::UpdateName(part_id, name) => {
            spare_part::update_name(&app_state.pool, &part_id, &name).await?;
            window.emit("update_spare_part_name", (part_id, name))?
        }
    }

    Ok(())
}

pub async fn update_machine(
    app_state: &AppState,
    mac: UpdateMachine,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match mac {
        UpdateMachine::UpdateName(machine_id, name) => {
            machine::update_name(&app_state.pool, &machine_id, &name).await?;
            window.emit("update_machine_name", (machine_id, name))?
        }
    }

    Ok(())
}

pub async fn update_department_shift(
    app_state: &AppState,
    shift: UpdateDepartmentShift,
    env: Env,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match shift {
        UpdateDepartmentShift::SaveShiftEmployee(shift_id, employee_id) => {
            shift_employee::save(&app_state.pool, shift_id, employee_id, env).await?;
            window.emit("update_shift_add_employee", (shift_id, employee_id))?
        }
        UpdateDepartmentShift::DeleteShiftEmployee(shift_id, employee_id) => {
            shift_employee::delete(&app_state.pool, shift_id, employee_id).await?;
            window.emit("update_shift_delete_employee", (shift_id, employee_id))?
        }
        UpdateDepartmentShift::SaveNote(note) => {
            note::save_to_shift(&app_state.pool, &note, env).await?;
            window.emit("update_shift_add_note", (note.shift_id, note.id))?
        }
        UpdateDepartmentShift::DeleteNote(shift_id, note_id) => {
            note::delete_shift_note(&app_state.pool, note_id).await?;
            window.emit("update_shift_delete_note", (shift_id, note_id))?
        }
        UpdateDepartmentShift::UpdateNote(note) => {
            note::update_shift_note(&app_state.pool, &note).await?;
            window.emit("update_shift_update_note", (note.id, note.content))?
        }
    }

    Ok(())
}

pub async fn update_department(
    app_state: &AppState,
    dep: UpdateDepartment,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match dep {
        UpdateDepartment::ChangeBoss(boss_id) => {
            department::change_boss(&app_state.pool, &boss_id).await?;
            let department_id = crate::memory::employee::find_employee_department_id_by_id(
                &app_state.pool,
                &boss_id,
            )
            .await?;
            window.emit("update_department_change_boss", (department_id, boss_id))?
        }
        UpdateDepartment::SetBoss(department_id, employee_id) => {
            department::set_boss(&app_state.pool, &department_id, &employee_id).await?;
            window.emit("update_department_set_boss", (department_id, employee_id))?
        }
        UpdateDepartment::UpdateName(department_id, name) => {
            department::update_name(&app_state.pool, &department_id, &name).await?;
            window.emit("update_department_name", (department_id, name))?
        }
    }
    Ok(())
}

pub async fn update_employee(
    app_state: &AppState,
    emp: UpdateEmployee,
    env: Env,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match emp {
        UpdateEmployee::AllowPermission(employee_id, permission) => {
            permissions::allow_permission(&app_state.pool, &employee_id, &permission, env).await?;
            window.emit(
                "update_employee_allow_permission",
                (employee_id, permission),
            )?
        }
        UpdateEmployee::ForbidPermission(employee_id, permission) => {
            permissions::forbid_permission(&app_state.pool, &employee_id, &permission).await?;
            window.emit(
                "update_employee_forbid_permission",
                (employee_id, permission),
            )?
        }
        UpdateEmployee::ForbidAllPermissions(employee_id) => {
            permissions::forbid_all_permissions(&app_state.pool, &employee_id).await?;
            window.emit("update_employee_forbid_all_permissions", employee_id)?
        }
        UpdateEmployee::Up(employee_id) => {
            employee::up(&app_state.pool, employee_id).await?;
            window.emit("update_employee_up", employee_id)?
        }
        UpdateEmployee::Down(employee_id) => {
            employee::down(&app_state.pool, employee_id).await?;
            window.emit("update_employee_down", employee_id)?
        }
        UpdateEmployee::UpdateDepartment(employee_id, department_id) => {
            employee::update_department(&app_state.pool, &employee_id, &department_id).await?;
            window.emit("update_employee_department", (employee_id, department_id))?
        }
        UpdateEmployee::UpdatePassword(employee_id, password) => {
            employee::update_password(&app_state.pool, &employee_id, &password).await?;
            window.emit("update_employee_password", (employee_id, password))?
        }
    }

    Ok(())
}
