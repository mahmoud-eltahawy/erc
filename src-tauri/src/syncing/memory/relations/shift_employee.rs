use std::error::Error;
use uuid::Uuid;

use sqlx::{query, Pool, Sqlite};

pub async fn save(
    pool: &Pool<Sqlite>,
    shift_id: Uuid,
    employee_id: Uuid,
) -> Result<(), Box<dyn Error>> {
    let shift_id = shift_id.to_string();
    let employee_id = employee_id.to_string();

    let row = query!(
        "
    INSERT OR IGNORE INTO department_shift_employee(
        department_shift_id,
        employee_id)
    VALUES($1,$2);",
        shift_id,
        employee_id
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete(
    pool: &Pool<Sqlite>,
    shift_id: Uuid,
    employee_id: Uuid,
) -> Result<(), Box<dyn Error>> {
    let shift_id = shift_id.to_string();
    let employee_id = employee_id.to_string();

    let row = query!(
        "
    DELETE FROM department_shift_employee
    WHERE department_shift_id = $1
    AND employee_id = $2;",
        shift_id,
        employee_id
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
