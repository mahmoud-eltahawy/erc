use sqlx::query;
use sqlx::{Pool, Sqlite};

pub async fn fetch_shift_problem_spare_parts(
    pool: &Pool<Sqlite>,
    shift_problem_id: &String,
) -> Result<Vec<String>, sqlx::Error> {
    let row = query!(
        "
    SELECT spare_part_id FROM shift_problem_spare_part
    WHERE shift_problem_id = $1",
        shift_problem_id
    )
    .fetch_all(pool);
    match row.await {
        Ok(spare_parts_ids_records) => Ok(spare_parts_ids_records
            .into_iter()
            .map(|sp| sp.spare_part_id)
            .collect()),
        Err(err) => Err(err),
    }
}

pub async fn fetch_shift_problem_problems(
    pool: &Pool<Sqlite>,
    shift_problem_id: &String,
) -> Result<Vec<String>, sqlx::Error> {
    let row = query!(
        "
    SELECT problem_id FROM shift_problem_problem
    WHERE shift_problem_id = $1",
        shift_problem_id
    )
    .fetch_all(pool);

    match row.await {
        Ok(spare_parts_ids_records) => Ok(spare_parts_ids_records
            .into_iter()
            .map(|sp| sp.problem_id)
            .collect()),
        Err(err) => Err(err),
    }
}
