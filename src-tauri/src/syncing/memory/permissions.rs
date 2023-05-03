use rec::model::permissions::PermissionName;
use sqlx::{query, Error, Pool, Sqlite};
use uuid::Uuid;

pub async fn allow_permission(
    pool: &Pool<Sqlite>,
    employee_id: &Uuid,
    permission: &PermissionName,
) -> Result<(), Error> {
    let employee_id = employee_id.to_string();
    let permission = permission.stringify();
    let row = query!(
        "
    INSERT OR IGNORE INTO
    permissions(employee_id,permission)
    VALUES($1,$2);",
        employee_id,
        permission
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
