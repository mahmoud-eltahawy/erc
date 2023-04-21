use rec::model::shift_problem::ShiftProblem;
use sqlx::{query, Error, Pool, Sqlite};
use uuid::Uuid;

pub async fn delete(pool: &Pool<Sqlite>, id: &Uuid) -> Result<(), Error> {
    let id = id.to_string();
    match query!(
        r#"
    DELETE FROM shift_problem WHERE id = $1;
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

pub async fn save(pool: &Pool<Sqlite>, problem: &ShiftProblem) -> Result<(), Error> {
    let ShiftProblem {
        id,
        shift_id,
        writer_id,
        maintainer_id,
        machine_id,
        begin_time,
        end_time,
    } = problem;
    let id = id.to_string();
    let shift_id = shift_id.to_string();
    let writer_id = writer_id.to_string();
    let maintainer_id = maintainer_id.to_string();
    let machine_id = machine_id.to_string();
    let begin_time = serde_json::json!(begin_time).to_string();
    let end_time = serde_json::json!(end_time).to_string();
    match sqlx::query!(
        r#"
    INSERT INTO shift_problem(id,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time)
    VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT (id) DO NOTHING;
  "#,
        id,
        shift_id,
        writer_id,
        maintainer_id,
        machine_id,
        begin_time,
        end_time
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn update(pool: &Pool<Sqlite>, problem: &ShiftProblem) -> Result<(), Error> {
    let ShiftProblem {
        id,
        shift_id,
        writer_id,
        maintainer_id,
        machine_id,
        begin_time,
        end_time,
    } = problem;
    let id = id.to_string();
    let shift_id = shift_id.to_string();
    let writer_id = writer_id.to_string();
    let maintainer_id = maintainer_id.to_string();
    let machine_id = machine_id.to_string();
    let begin_time = serde_json::json!(begin_time).to_string();
    let end_time = serde_json::json!(end_time).to_string();
    match query!(
        r#"
    UPDATE shift_problem SET
    shift_id        = $2,
    writer_id       = $3,
    maintainer_id   = $4,
    machine_id      = $5,
    begin_time      = $6,
    end_time        = $7
    WHERE id        = $1;
  "#,
        id,
        shift_id,
        writer_id,
        maintainer_id,
        machine_id,
        begin_time,
        end_time
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
