use chrono::NaiveTime;
use rec::model::shift_problem::ShiftProblem;
use sqlx::{query, Error, Pool, Sqlite};
use uuid::Uuid;

use crate::syncing::Env;

pub async fn delete(pool: &Pool<Sqlite>, id: &Uuid) -> Result<(), Error> {
    let id = id.to_string();
    match query!(
        r#"
    DELETE FROM shift_problem WHERE id = $1;
  "#,
        id
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn save(pool: &Pool<Sqlite>, problem: &ShiftProblem, env: Env) -> Result<(), Error> {
    let ShiftProblem {
        id,
        shift_id,
        maintainer_id,
        machine_id,
        begin_time,
        end_time,
    } = problem;
    let (updater_id, time_stamp) = env;
    let id = id.to_string();
    let shift_id = shift_id.to_string();
    let updater_id = updater_id.to_string();
    let maintainer_id = maintainer_id.to_string();
    let machine_id = machine_id.to_string();
    let time_stamp = serde_json::json!(time_stamp).to_string();
    let begin_time = serde_json::json!(begin_time).to_string();
    let end_time = serde_json::json!(end_time).to_string();
    match sqlx::query!(
        r#"
    INSERT INTO shift_problem(id,shift_id,maintainer_id,
        machine_id,begin_time,end_time,updater_id,time_stamp)
      VALUES($1,$2,$3,$4,$5,$6,$7,$8) ON CONFLICT (id) DO NOTHING;
  "#,
        id,
        shift_id,
        maintainer_id,
        machine_id,
        begin_time,
        end_time,
        updater_id,
        time_stamp
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn update_end_time(
    pool: &Pool<Sqlite>,
    id: Uuid,
    end_time: NaiveTime,
) -> Result<(), Error> {
    let id = id.to_string();
    let end_time = serde_json::json!(end_time).to_string();
    match query!(
        r#"
    UPDATE shift_problem SET
    end_time   = $2
    WHERE id   = $1;
  "#,
        id,
        end_time,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn update_begin_time(
    pool: &Pool<Sqlite>,
    id: Uuid,
    begin_time: NaiveTime,
) -> Result<(), Error> {
    let id = id.to_string();
    let begin_time = serde_json::json!(begin_time).to_string();
    match query!(
        r#"
    UPDATE shift_problem SET
    begin_time   = $2
    WHERE id     = $1;
  "#,
        id,
        begin_time,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn update_maintainer(
    pool: &Pool<Sqlite>,
    id: Uuid,
    maintainer_id: Uuid,
) -> Result<(), Error> {
    let id = id.to_string();
    let maintainer_id = maintainer_id.to_string();
    match query!(
        r#"
    UPDATE shift_problem SET
    maintainer_id   = $2
    WHERE id        = $1;
  "#,
        id,
        maintainer_id,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn update_machine(pool: &Pool<Sqlite>, id: Uuid, machine_id: Uuid) -> Result<(), Error> {
    let id = id.to_string();
    let machine_id = machine_id.to_string();
    match query!(
        r#"
    UPDATE shift_problem SET
    machine_id   = $2
    WHERE id     = $1;
  "#,
        id,
        machine_id,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
