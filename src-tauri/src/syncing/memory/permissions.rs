use rec::model::permissions::PermissionName;
use sqlx::{query, Error, Pool, Sqlite};
use uuid::Uuid;

use crate::syncing::Env;

pub async fn allow_permission(
    pool: &Pool<Sqlite>,
    employee_id: &Uuid,
    permission: &PermissionName,
    env: Env,
) -> Result<(), Error> {
    let (updater_id, time_stamp) = env;
    let employee_id = employee_id.to_string();
    let permission = permission.stringify();

    let updater_id = updater_id.to_string();
    let time_stamp = serde_json::json!(time_stamp).to_string();
    let row = query!(
        "
    INSERT OR IGNORE INTO
    permissions(employee_id,permission,updater_id,time_stamp)
    VALUES($1,$2,$3,$4);",
        employee_id,
        permission,
        updater_id,
        time_stamp
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn forbid_permission(
    pool: &Pool<Sqlite>,
    employee_id: &Uuid,
    permission: &PermissionName,
) -> Result<(), Error> {
    let employee_id = employee_id.to_string();
    let permission = permission.stringify();
    let row = query!(
        "
    DELETE FROM permissions
    WHERE employee_id = $1
    AND permission = $2;",
        employee_id,
        permission
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn forbid_all_permissions(pool: &Pool<Sqlite>, employee_id: &Uuid) -> Result<(), Error> {
    let employee_id = employee_id.to_string();
    let row = query!(
        "
    DELETE FROM permissions
    WHERE employee_id = $1;",
        employee_id,
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
