mod cds;
mod memory;
mod updates;

use cds::*;
use chrono::NaiveDateTime;
use updates::*;
use uuid::Uuid;

use crate::{api, config::AppState};
use std::error::Error;

use rec::{
    crud_sync::{CdVersion, Table, UpdateVersion, Version},
    model::Update,
};
use tauri::Window;

use memory::syncing;

pub type Env = (Uuid, NaiveDateTime);

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
        target_id: _,
        updater_id,
        time_stamp,
        json,
    } = version;
    let env = (updater_id, time_stamp);
    match json {
        Update::Employee(emp) => update_employee(app_state, emp, env, window).await?,
        Update::Department(dep) => update_department(app_state, dep, window).await?,
        Update::DepartmentShift(shift) => {
            update_department_shift(app_state, shift, env, window).await?
        }
        Update::Machine(mac) => update_machine(app_state, mac, window).await?,
        Update::Problem(pro) => update_problem(app_state, pro, window).await?,
        Update::ShiftProblem(sp) => update_shift_problem(app_state, sp, env, window).await?,
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
        target_id,
        updater_id,
        time_stamp,
        target_table,
    } = version;
    let env = (updater_id, time_stamp);
    match target_table {
        Table::Employee => cd_employee(app_state, cd, target_id, env, window).await?,
        Table::Problem => cd_problem(app_state, cd, target_id, env, window).await?,
        Table::SparePart => cd_spare_part(app_state, cd, target_id, env, window).await?,
        Table::Machine => cd_machine(app_state, cd, target_id, env, window).await?,
        Table::ShiftProblem => cd_shift_problem(app_state, cd, target_id, env, window).await?,
        Table::Shift => cd_shift(app_state, cd, target_id, env).await?,
        Table::Department => cd_department(app_state, cd, target_id, env, window).await?,
        Table::DepartmentShift => cd_department_shift(app_state, cd, target_id, env).await?,
    }
    syncing::save_cd_version(&app_state.pool, version_number).await?;
    Ok(())
}
