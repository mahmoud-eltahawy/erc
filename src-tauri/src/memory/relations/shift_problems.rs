use std::str::FromStr;

use sqlx::query;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

pub async fn fetch_shift_problem_spare_parts_ids(
    pool: &Pool<Sqlite>,
    shift_problem_id: &Uuid,
) -> Result<Vec<Uuid>, sqlx::Error> {
    let shift_problem_id = shift_problem_id.to_string();
    let row = query!(
        "
        SELECT id FROM spare_part WHERE id IN (
            SELECT spare_part_id FROM shift_problem_spare_part
            WHERE shift_problem_id = $1
        )
    ",
        shift_problem_id
    )
    .fetch_all(pool);
    match row.await {
        Ok(records) => Ok(records
            .into_iter()
            .flat_map(|r| Uuid::from_str(&r.id))
            .collect()),
        Err(err) => Err(err),
    }
}

pub async fn fetch_shift_problem_problems_ids(
    pool: &Pool<Sqlite>,
    shift_problem_id: &Uuid,
) -> Result<Vec<Uuid>, sqlx::Error> {
    let shift_problem_id = shift_problem_id.to_string();
    let row = query!(
        "
    SELECT p.id from problem p WHERE p.id in (
        SELECT spp.problem_id FROM shift_problem_problem spp
        WHERE spp.shift_problem_id = $1
   )",
        shift_problem_id
    )
    .fetch_all(pool);

    match row.await {
        Ok(records) => Ok(records
            .into_iter()
            .flat_map(|r| Uuid::from_str(&r.id))
            .collect()),
        Err(err) => Err(err),
    }
}
