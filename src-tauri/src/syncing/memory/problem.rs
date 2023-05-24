use rec::model::problem::Problem;
use sqlx::{query, Error, Pool, Sqlite};
use uuid::Uuid;

use crate::syncing::Env;

pub async fn delete(pool: &Pool<Sqlite>, id: Uuid) -> Result<(), Error> {
    let id = id.to_string();
    match query!(
        r#"
    DELETE FROM problem WHERE id = $1;
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

pub async fn save(pool: &Pool<Sqlite>, problem: Problem, env: Env) -> Result<(), Error> {
    let Problem {
        id,
        department_id,
        title,
        description,
    } = problem;
    let (updater_id, time_stamp) = env;
    let id = id.to_string();
    let department_id = department_id.to_string();
    let updater_id = updater_id.to_string();
    let time_stamp = serde_json::json!(time_stamp).to_string();
    match query!(
        r#"
    INSERT INTO problem(id,department_id,title,description,updater_id,time_stamp)
    VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT (id) DO NOTHING;
  "#,
        id,
        department_id,
        title,
        description,
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

pub async fn update_description(
    pool: &Pool<Sqlite>,
    problem_id: &Uuid,
    description: &String,
) -> Result<(), Error> {
    let problem_id = problem_id.to_string();
    match query!(
        r#"
  UPDATE problem SET
  description = $2
  WHERE id    = $1;
  "#,
        problem_id,
        description,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn update_title(
    pool: &Pool<Sqlite>,
    problem_id: &Uuid,
    title: &String,
) -> Result<(), Error> {
    let problem_id = problem_id.to_string();
    match query!(
        r#"
  UPDATE problem SET
  title    = $2
  WHERE id = $1;
  "#,
        problem_id,
        title,
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
