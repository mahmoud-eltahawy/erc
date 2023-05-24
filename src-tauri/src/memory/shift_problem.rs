use std::str::FromStr;

use itertools::Itertools;
use rec::model::shift_problem::ShiftProblem;
use sqlx::{query, Pool, Sqlite};
use uuid::Uuid;

type Error = Box<dyn std::error::Error>;

pub async fn find_shift_shift_problems_ids(
    pool: &Pool<Sqlite>,
    shift_id: Uuid,
) -> Result<Vec<Uuid>, Error> {
    let shift_id = shift_id.to_string();
    let records = query!(
        r#"
        select id from shift_problem
        WHERE shift_id = $1;
    "#,
        shift_id
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|r| Uuid::from_str(&r.id))
        .collect_vec())
}

pub async fn find_shift_problem_shift_id(pool: &Pool<Sqlite>, id: &Uuid) -> Result<Uuid, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
        select shift_id from shift_problem
        WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(Uuid::from_str(&record.shift_id)?)
}

pub async fn find_shift_problem_by_id(
    pool: &Pool<Sqlite>,
    id: Uuid,
) -> Result<ShiftProblem, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
      SELECT id,machine_id,maintainer_id,shift_id,begin_time,end_time FROM shift_problem WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await?;
    let id = Uuid::from_str(&record.id)?;
    let machine_id = Uuid::from_str(&record.machine_id)?;
    let maintainer_id = Uuid::from_str(&record.maintainer_id)?;
    let shift_id = Uuid::from_str(&record.shift_id)?;
    let begin_time = serde_json::from_str(&record.begin_time)?;
    let end_time = serde_json::from_str(&record.end_time)?;
    Ok(ShiftProblem {
        id,
        shift_id,
        maintainer_id,
        machine_id,
        begin_time,
        end_time,
    })
}
