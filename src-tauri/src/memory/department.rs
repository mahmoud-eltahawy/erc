use std::str::FromStr;

use itertools::Itertools;
use rec::model::{department::Department, name::Name};
use sqlx::{query, Pool, Sqlite};
use uuid::Uuid;

type Error = Box<dyn std::error::Error>;

pub async fn find_all_departments(pool: &Pool<Sqlite>) -> Result<Vec<Name>, Error> {
    let records = query!(
        r#"
      select id,name from department WHERE id <> '00000000-0000-0000-0000-000000000000';
    "#
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| match Uuid::from_str(&record.id) {
            Ok(id) => Some(Name {
                id,
                name: record.name,
            }),
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_department_by_id(pool: &Pool<Sqlite>, id: Uuid) -> Result<Department, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
      SELECT * FROM department WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await?;
    let id = Uuid::from_str(&record.id)?;
    let boss_id = record.boss_id.map(|x| match Uuid::from_str(&x) {
        Ok(id) => id,
        Err(_) => Uuid::nil(), //TODO change this later
    });
    Ok(Department {
        id,
        boss_id,
        name: record.name,
    })
}

pub async fn find_employee_department_id_and_boss_id(
    pool: &Pool<Sqlite>,
    id: Uuid,
) -> Result<(Uuid, Option<Uuid>), Error> {
    let id = id.to_string();
    let record = query!(
        r#"
      SELECT d.id,d.boss_id FROM department d WHERE d.id = (
        SELECT e.department_id FROM employee e WHERE e.id = $1);
    "#,
        id
    )
    .fetch_one(pool)
    .await?;
    let id = Uuid::from_str(&record.id)?;
    let Some(boss_id) = record.boss_id else {
        return Ok((id,None));
    };
    let boss_id = Uuid::from_str(&boss_id)?;
    Ok((id, Some(boss_id)))
}

pub async fn find_department_name_by_id(pool: &Pool<Sqlite>, id: Uuid) -> Result<String, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
      SELECT name FROM department WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(record.name)
}

pub async fn find_department_boss_id_by_id(
    pool: &Pool<Sqlite>,
    id: Uuid,
) -> Result<Option<Uuid>, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
      SELECT boss_id FROM department WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await?;
    let id = match record.boss_id {
        Some(id) => Some(Uuid::from_str(&id)?),
        None => None,
    };
    Ok(id)
}
