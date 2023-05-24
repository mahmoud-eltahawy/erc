use rec::model::machine::Machine;
use sqlx::{query, Error, Pool, Sqlite};
use uuid::Uuid;

use crate::syncing::Env;

pub async fn delete(pool: &Pool<Sqlite>, id: Uuid) -> Result<(), Error> {
    let id = id.to_string();
    match query!(
        r#"
    DELETE FROM machine WHERE id = $1;
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

pub async fn save(pool: &Pool<Sqlite>, machine: Machine, env: Env) -> Result<(), Error> {
    let Machine { id, name } = machine;
    let (updater_id, time_stamp) = env;
    let id = id.to_string();
    let updater_id = updater_id.to_string();
    let time_stamp = serde_json::json!(time_stamp).to_string();
    match query!(
        r#"
    INSERT INTO machine(id,name,updater_id,time_stamp)
    VALUES($1,$2,$3,$4) ON CONFLICT (id) DO NOTHING;
  "#,
        id,
        name,
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

pub async fn update_name(
    pool: &Pool<Sqlite>,
    machine_id: &Uuid,
    name: &String,
) -> Result<(), Error> {
    let machine_id = machine_id.to_string();
    match query!(
        r#"
  UPDATE machine SET
  name = $2
  WHERE id = $1;
  "#,
        machine_id,
        name
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
