use std::str::FromStr;

use rec::model::name::Name;
use sqlx::query;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

pub async fn fetch_shift_problem_spare_parts_names(
    pool: &Pool<Sqlite>,
    shift_problem_id: &Uuid,
) -> Result<Vec<Name<Uuid>>, sqlx::Error> {
    let shift_problem_id = shift_problem_id.to_string();
    let row = query!(
        "
        SELECT id,name FROM spare_part WHERE id IN (
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
            .flat_map(|r| match Uuid::from_str(&r.id) {
                Ok(id) => Some(Name { id, name: r.name }),
                Err(_) => None,
            })
            .collect()),
        Err(err) => Err(err),
    }
}

pub async fn fetch_shift_problem_problems_names(
    pool: &Pool<Sqlite>,
    shift_problem_id: &Uuid,
) -> Result<Vec<Name<Uuid>>, sqlx::Error> {
    let shift_problem_id = shift_problem_id.to_string();
    let row = query!(
        "
    SELECT id,title from problem WHERE id in (
        SELECT problem_id FROM shift_problem_problem
        WHERE shift_problem_id = $1
   )",
        shift_problem_id
    )
    .fetch_all(pool);

    match row.await {
        Ok(records) => Ok(records
            .into_iter()
            .flat_map(|r| match Uuid::from_str(&r.id) {
                Ok(id) => Some(Name { id, name: r.title }),
                Err(_) => None,
            })
            .collect()),
        Err(err) => Err(err),
    }
}
