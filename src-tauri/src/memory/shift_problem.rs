use std::str::FromStr;

use itertools::Itertools;
use rec::model::shift_problem::ClientShiftProblem;
use sqlx::{query, query_as, Error, Pool, Sqlite};
use uuid::Uuid;

pub async fn find_shift_shift_problems_ids(
    pool: &Pool<Sqlite>,
    shift_id: Uuid,
) -> Result<Vec<Uuid>, Error> {
    let shift_id = shift_id.to_string();
    match query!(
        r#"
        select id from shift_problem
        WHERE shift_id = $1;
    "#,
        shift_id
    )
    .fetch_all(pool)
    .await
    {
        Ok(record) => Ok(record
            .into_iter()
            .flat_map(|r| Uuid::from_str(r.id.as_str()))
            .collect_vec()),
        Err(err) => Err(err),
    }
}

pub async fn find_shift_problem_shift_id(pool: &Pool<Sqlite>, id: &Uuid) -> Result<Uuid, Error> {
    let id = id.to_string();
    match query!(
        r#"
        select shift_id from shift_problem
        WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(record) => match Uuid::from_str(&record.shift_id) {
            Ok(id) => Ok(id),
            Err(_) => Err(Error::RowNotFound),
        },
        Err(err) => Err(err),
    }
}

pub async fn find_shift_problem_by_id(
    pool: &Pool<Sqlite>,
    id: Uuid,
) -> Result<ClientShiftProblem, Error> {
    let id = id.to_string();
    match query_as!(
        ClientShiftProblem,
        r#"
      SELECT * FROM shift_problem WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(problem) => Ok(problem),
        Err(err) => Err(err),
    }
}
