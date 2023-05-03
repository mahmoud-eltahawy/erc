mod cds;
mod memory;
mod updates;

use cds::*;
use updates::*;

use crate::{api, config::AppState};
use std::error::Error;

use rec::{
    crud_sync::{CdVersion, Table, UpdateVersion, Version},
    model::Update,
};
use tauri::Window;

use memory::syncing;

pub async fn continious_upgrade(app_state: AppState, window: Window) {
    loop {
        upgrade(&app_state, &window).await;
    }
}

pub async fn upgrade(app_state: &AppState, window: &Window) {
    let Ok(version) = syncing::last_version(&app_state.pool).await else {
        return;
    };
    let Ok(updates) = api::fetch_last_versions(app_state, version).await else {
        return;
    };
    let Ok(_) = window.emit("begin_major_update", updates.len()) else { return;};
    let mut successes = 0;
    let mut failures = 0;
    for update in updates {
        match apply_update(app_state, update, window).await {
            Ok(_) => successes += 1,
            Err(_) => failures += 1,
        }
        let Ok(_) = window.emit("major_update_step",(successes,failures)) else { return;};
    }
    let Ok(_) = window.emit("end_major_update", None::<i32>) else { return;};
}

async fn apply_update(
    app_state: &AppState,
    version: Version,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    match version {
        Version::Cd(cd) => apply_cd_version_update(app_state, cd, window).await?,
        Version::Update(update) => apply_update_version_update(app_state, update, window).await?,
    }
    Ok(())
}

async fn apply_update_version_update(
    app_state: &AppState,
    version: UpdateVersion,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    let UpdateVersion {
        version_number,
        target_id,
        updater_id,
        time_stamp,
        json,
    } = version;
    match json {
        Update::Employee(emp) => update_employee(app_state, emp, window).await?,
        Update::Department(dep) => update_department(app_state, dep, window).await?,
        Update::DepartmentShift(shift) => update_department_shift(app_state, shift, window).await?,
        Update::Machine(mac) => update_machine(app_state, mac, window).await?,
        Update::Problem(pro) => update_problem(app_state, pro, window).await?,
        Update::ShiftProblem(sp) => update_shift_problem(app_state, sp, window).await?,
        Update::SparePart(part) => update_spare_part(app_state, part, window).await?,
    }
    syncing::save_update_version(&app_state.pool, version_number).await?;
    Ok(())
}

async fn apply_cd_version_update(
    app_state: &AppState,
    version: CdVersion,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    let CdVersion {
        version_number,
        cd,
        time_stamp,
        updater_id,
        target_id,
        target_table,
        other_target_id,
    } = version;
    match target_table {
        Table::Employee => cd_employee(app_state, cd, target_id, window).await?,
        Table::Problem => cd_problem(app_state, cd, target_id, window).await?,
        Table::SparePart => cd_spare_part(app_state, cd, target_id, window).await?,
        Table::Machine => cd_machine(app_state, cd, target_id, window).await?,
        Table::ShiftProblem => cd_shift_problem(app_state, cd, target_id, window).await?,
        Table::Shift => cd_shift(app_state, cd, target_id).await?,
        Table::Department => cd_department(app_state, cd, target_id, window).await?,
        Table::DepartmentShift => cd_department_shift(app_state, cd, target_id).await?,
    }
    syncing::save_cd_version(&app_state.pool, version_number).await?;
    Ok(())
}
