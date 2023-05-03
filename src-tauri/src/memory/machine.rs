use std::str::FromStr;

use itertools::Itertools;
use rec::model::name::Name;
use sqlx::{query, Pool, Sqlite};
use uuid::Uuid;

type Error = Box<dyn std::error::Error>;

pub async fn find_all_machines(pool: &Pool<Sqlite>) -> Result<Vec<Name>, Error> {
    let records = query!(
        r#"
      select * from machine;
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

pub async fn find_machines_by_name(
    pool: &Pool<Sqlite>,
    target: &str,
    canceled: Vec<String>,
) -> Result<Vec<Name>, Error> {
    let target = format!("%{target}%");
    let limit = 4;
    let limit = limit + canceled.len() as i64;
    let records = query!(
        "
    SELECT * FROM machine
    WHERE name LIKE $1 LIMIT $2;",
        target,
        limit
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| match Uuid::from_str(&record.id) {
            Ok(id) => {
                if !canceled.contains(&record.name) {
                    return Some(Name {
                        id,
                        name: record.name,
                    });
                } else {
                    return None;
                }
            }
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_4_machines(
    pool: &Pool<Sqlite>,
    canceled: Vec<String>,
) -> Result<Vec<Name>, Error> {
    let limit = 4;
    let limit = limit + canceled.len() as i64;
    let records = query!(
        "
    SELECT * FROM machine LIMIT $1;",
        limit
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| match Uuid::from_str(&record.id) {
            Ok(id) => {
                if !canceled.contains(&record.name) {
                    return Some(Name {
                        id,
                        name: record.name,
                    });
                } else {
                    return None;
                }
            }
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_all_machines_names(pool: &Pool<Sqlite>) -> Result<Vec<Name>, Error> {
    let records = query!(
        r#"
      select id,name from machine;
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

pub async fn find_machine_name_by_id(pool: &Pool<Sqlite>, id: Uuid) -> Result<String, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
      SELECT name FROM machine WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(record.name)
}
