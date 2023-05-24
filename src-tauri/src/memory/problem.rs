use std::str::FromStr;

use itertools::Itertools;
use rec::model::{name::Name, problem::Problem};
use sqlx::{query, Pool, Sqlite};
use uuid::Uuid;

type Error = Box<dyn std::error::Error>;

pub async fn find_all_problems(pool: &Pool<Sqlite>) -> Result<Vec<Problem>, Error> {
    let records = query!(
        r#"
    select id,department_id,title,description from problem;
  "#
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| {
            match (
                Uuid::from_str(&record.id),
                Uuid::from_str(&record.department_id),
            ) {
                (Ok(id), Ok(department_id)) => Some(Problem {
                    id,
                    department_id,
                    title: record.title,
                    description: record.description,
                }),
                _ => None,
            }
        })
        .collect_vec())
}

pub async fn find_department_all_problems(
    pool: &Pool<Sqlite>,
    department_id: Uuid,
) -> Result<Vec<Name>, Error> {
    let department_id = department_id.to_string();
    let records = query!(
        r#"
    select id,title as name from problem WHERE department_id = $1;
  "#,
        department_id
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
            _ => None,
        })
        .collect_vec())
}

pub async fn find_problem_name_by_id(pool: &Pool<Sqlite>, id: Uuid) -> Result<String, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
    SELECT title FROM problem WHERE id = $1;
  "#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(record.title)
}

pub async fn find_problems_by_department_id(
    pool: &Pool<Sqlite>,
    id: Uuid,
) -> Result<Vec<Name>, Error> {
    let id = id.to_string();
    let records = query!(
        r#"
    SELECT id , title as name FROM problem WHERE department_id = $1 LIMIT 7;
  "#,
        id
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
            _ => None,
        })
        .collect_vec())
}

pub async fn find_department_problems_by_name(
    pool: &Pool<Sqlite>,
    department_id: Uuid,
    target: &str,
    canceled: Vec<String>,
    limit: i64,
) -> Result<Vec<Name>, Error> {
    let department_id = department_id.to_string();
    let limit = limit + canceled.len() as i64;
    let target = format!("%{target}%");
    let records = query!(
        "
    SELECT id,title as name FROM problem
    WHERE department_id = $1
    AND title LIKE $2
    LIMIT $3;",
        department_id,
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

pub async fn find_department_full_problems_by_name(
    pool: &Pool<Sqlite>,
    department_id: Uuid,
    target: &str,
) -> Result<Vec<Name>, Error> {
    let target = format!("%{target}%");
    let department_id = department_id.to_string();
    let records = query!(
        r#"
    SELECT id ,title as name FROM problem
    WHERE department_id = $1
    AND title LIKE $2
    LIMIT 8;"#,
        department_id,
        target
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

pub async fn find_department_limit_of_problems(
    pool: &Pool<Sqlite>,
    department_id: Uuid,
    canceled: Vec<String>,
    limit: i64,
) -> Result<Vec<Name>, Error> {
    let department_id = department_id.to_string();
    let limit = limit + canceled.len() as i64;
    let records = query!(
        "
    SELECT id,title as name FROM problem
    WHERE department_id = $1
    LIMIT $2;",
        department_id,
        limit
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| match Uuid::from_str(&record.id) {
            Ok(id) => {
                if !canceled.contains(&record.name) {
                    Some(Name {
                        id,
                        name: record.name,
                    })
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect_vec())
}

pub async fn find_problems_by_writer_id(
    pool: &Pool<Sqlite>,
    id: Uuid,
) -> Result<Vec<Problem>, Error> {
    let id = id.to_string();
    let records = query!(
        r#"
    SELECT id,department_id,title,description FROM problem WHERE updater_id = $1;
  "#,
        id
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| {
            match (
                Uuid::from_str(&record.id),
                Uuid::from_str(&record.department_id),
            ) {
                (Ok(id), Ok(department_id)) => Some(Problem {
                    id,
                    department_id,
                    title: record.title,
                    description: record.description,
                }),
                _ => None,
            }
        })
        .collect_vec())
}

pub async fn find_problem_by_id(pool: &Pool<Sqlite>, id: Uuid) -> Result<Problem, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
    SELECT id,department_id,title,description FROM problem WHERE id = $1;
  "#,
        id
    )
    .fetch_one(pool)
    .await?;
    match (
        Uuid::from_str(&record.id),
        Uuid::from_str(&record.department_id),
    ) {
        (Ok(id), Ok(department_id)) => Ok(Problem {
            id,
            department_id,
            title: record.title,
            description: record.description,
        }),
        _ => Err("err".into()),
    }
}
