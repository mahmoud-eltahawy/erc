use rec::model::employee::Employee;
use sqlx::{query, Error, Pool, Sqlite};
use uuid::Uuid;

use crate::syncing::Env;

pub async fn delete(pool: &Pool<Sqlite>, id: Uuid) -> Result<(), Error> {
    let id = id.to_string();
    match query!(
        r#"
    DELETE FROM employee WHERE id = $1;
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

pub async fn save(pool: &Pool<Sqlite>, employee: Employee, env: Env) -> Result<(), Error> {
    let Employee {
        id,
        card_id,
        department_id,
        first_name,
        last_name,
        middle_name,
        position,
        password,
    } = employee;
    let (updater_id, time_stamp) = env;
    let id = id.to_string();
    let department_id = department_id.to_string();
    let updater_id = updater_id.to_string();
    let time_stamp = serde_json::json!(time_stamp).to_string();
    match query!(
        r#"
    INSERT INTO employee(id,card_id,department_id,first_name,
        last_name,middle_name,position,password,updater_id,time_stamp)
    VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) ON CONFLICT (id) DO NOTHING;
  "#,
        id,
        card_id,
        department_id,
        first_name,
        last_name,
        middle_name,
        position,
        password,
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

pub async fn update_department(
    pool: &Pool<Sqlite>,
    employee_id: &Uuid,
    department_id: &Uuid,
) -> Result<(), Error> {
    let employee_id = employee_id.to_string();
    let department_id = department_id.to_string();
    match query!(
        r#"
  UPDATE employee SET
  department_id = $2
  WHERE id = $1;
  "#,
        employee_id,
        department_id,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn update_password(
    pool: &Pool<Sqlite>,
    employee_id: &Uuid,
    password: &String,
) -> Result<(), Error> {
    let employee_id = employee_id.to_string();
    match query!(
        r#"
  UPDATE employee SET
  password = $2
  WHERE id = $1;
  "#,
        employee_id,
        password,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn up(pool: &Pool<Sqlite>, employee_id: Uuid) -> Result<(), Error> {
    let employee_id = employee_id.to_string();
    match query!(
        r#"
  UPDATE employee SET
  position = 'SUPER_USER'
  WHERE id = $1;
  "#,
        employee_id,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn down(pool: &Pool<Sqlite>, employee_id: Uuid) -> Result<(), Error> {
    let employee_id = employee_id.to_string();
    match query!(
        r#"
  UPDATE employee SET
  position = 'USER'
  WHERE id = $1;
  "#,
        employee_id,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
