use std::{error::Error, str::FromStr};

use itertools::Itertools;
use rec::model::name::Name;
use sqlx::{query, Pool, Sqlite};
use uuid::Uuid;

pub async fn find_all_spare_parts(pool: &Pool<Sqlite>) -> Result<Vec<Name>, Box<dyn Error>> {
    let records = query!(
        r#"
      select * from spare_part;
    "#
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|x| match Uuid::from_str(&x.id) {
            Ok(id) => Some(Name { id, name: x.name }),
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_spare_parts_by_name(
    pool: &Pool<Sqlite>,
    target: &str,
    canceled: Vec<String>,
    limit: i64,
) -> Result<Vec<Name>, Box<dyn Error>> {
    let target = format!("%{target}%");
    let limit = limit + canceled.len() as i64;
    let records = query!(
        "
    SELECT * FROM spare_part
    WHERE name LIKE $1 LIMIT $2;",
        target,
        limit
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|x| match Uuid::from_str(&x.id) {
            Ok(id) => {
                if !canceled.contains(&x.name) {
                    return Some(Name { id, name: x.name });
                } else {
                    return None;
                }
            }
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_limit_of_spare_parts(
    pool: &Pool<Sqlite>,
    canceled: Vec<String>,
    limit: i64,
) -> Result<Vec<Name>, Box<dyn Error>> {
    let limit = limit + canceled.len() as i64;
    let records = query!(
        "
    SELECT * FROM spare_part LIMIT $1;",
        limit
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|x| match Uuid::from_str(&x.id) {
            Ok(id) => {
                if !canceled.contains(&x.name) {
                    return Some(Name { id, name: x.name });
                } else {
                    return None;
                }
            }
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_all_spare_parts_names(pool: &Pool<Sqlite>) -> Result<Vec<Name>, Box<dyn Error>> {
    let records = query!(
        "
    SELECT * FROM spare_part;"
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|x| match Uuid::from_str(&x.id) {
            Ok(id) => Some(Name { id, name: x.name }),
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_spare_part_name_by_id(
    pool: &Pool<Sqlite>,
    id: Uuid,
) -> Result<String, Box<dyn Error>> {
    let id = id.to_string();
    let record = query!(
        r#"
      SELECT name FROM spare_part WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(record.name)
}
