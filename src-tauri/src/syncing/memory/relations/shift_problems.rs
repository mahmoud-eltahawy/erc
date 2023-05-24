use sqlx::{Pool, Sqlite};
use std::error::Error;
use uuid::Uuid;

use sqlx::query;

use crate::syncing::Env;

pub async fn save_problem(
    pool: &Pool<Sqlite>,
    spid: Uuid,
    pid: Uuid,
    env: Env,
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    let shift_problem_id = spid.to_string();
    let problem_id = pid.to_string();

    let updater_id = updater_id.to_string();
    let time_stamp = serde_json::json!(time_stamp).to_string();
    let row = query!(
        "
    INSERT OR IGNORE INTO shift_problem_problem(
        shift_problem_id,
        problem_id,
        updater_id,
        time_stamp
    )
    VALUES($1,$2,$3,$4);",
        shift_problem_id,
        problem_id,
        updater_id,
        time_stamp
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_problem(
    pool: &Pool<Sqlite>,
    spid: Uuid,
    pid: Uuid,
) -> Result<(), Box<dyn Error>> {
    let shift_problem_id = spid.to_string();
    let problem_id = pid.to_string();

    let row = query!(
        "
    DELETE FROM shift_problem_problem
    WHERE shift_problem_id = $1 AND problem_id = $2;",
        shift_problem_id,
        problem_id
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn save_spare_part(
    pool: &Pool<Sqlite>,
    spid: Uuid,
    pid: Uuid,
    env: Env,
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    let shift_problem_id = spid.to_string();
    let spare_part_id = pid.to_string();

    let updater_id = updater_id.to_string();
    let time_stamp = serde_json::json!(time_stamp).to_string();
    let row = query!(
        "
    INSERT OR IGNORE INTO shift_problem_spare_part(
        shift_problem_id,
        spare_part_id,
        updater_id,
        time_stamp
    )
    VALUES($1,$2,$3,$4);",
        shift_problem_id,
        spare_part_id,
        updater_id,
        time_stamp
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_spare_part(
    pool: &Pool<Sqlite>,
    spid: Uuid,
    pid: Uuid,
) -> Result<(), Box<dyn Error>> {
    let shift_problem_id = spid.to_string();
    let spare_part_id = pid.to_string();

    let row = query!(
        "
    DELETE FROM shift_problem_spare_part
    WHERE shift_problem_id = $1 AND spare_part_id = $2;",
        shift_problem_id,
        spare_part_id
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
