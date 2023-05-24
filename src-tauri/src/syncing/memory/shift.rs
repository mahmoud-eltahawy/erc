use rec::model::shift::{DepartmentShift, Shift};
use sqlx::{query, Error, Pool, Sqlite};
use uuid::Uuid;

use crate::syncing::Env;

pub async fn delete_department_shift(pool: &Pool<Sqlite>, id: Uuid) -> Result<(), Error> {
    let id = id.to_string();
    match query!(
        r#"
    DELETE FROM department_shift WHERE id = $1;
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

pub async fn save_department_shift(
    pool: &Pool<Sqlite>,
    shift: DepartmentShift,
    env: Env,
) -> Result<(), Error> {
    let DepartmentShift {
        id,
        shift_id,
        department_id,
    } = shift;
    let (updater_id, time_stamp) = env;
    let id = id.to_string();
    let shift_id = shift_id.to_string();
    let department_id = department_id.to_string();

    let updater_id = updater_id.to_string();
    let time_stamp = serde_json::json!(time_stamp).to_string();
    match query!(
        r#"
    INSERT INTO department_shift(id,department_id,shift_id,updater_id,time_stamp)
    VALUES($1,$2,$3,$4,$5) ON CONFLICT (id) DO NOTHING;
  "#,
        id,
        department_id,
        shift_id,
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

pub async fn save(pool: &Pool<Sqlite>, shift: Shift, env: Env) -> Result<(), Error> {
    let Shift {
        id,
        shift_date,
        shift_order,
    } = shift;
    let (updater_id, time_stamp) = env;
    let id = id.to_string();
    let shift_date = serde_json::json!(shift_date);
    let shift_order = shift_order.stringify();

    let updater_id = updater_id.to_string();
    let time_stamp = serde_json::json!(time_stamp).to_string();
    match query!(
        r#"
    INSERT INTO shift(id,shift_date,shift_order,updater_id,time_stamp)
    VALUES($1,$2,$3,$4,$5) ON CONFLICT (id) DO NOTHING;
  "#,
        id,
        shift_date,
        shift_order,
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
