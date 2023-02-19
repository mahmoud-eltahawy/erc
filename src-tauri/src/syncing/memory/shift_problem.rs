use rec::model::shift_problem::{ClientShiftProblem, ShiftProblem};
use sqlx::{Pool, Sqlite,Error, query};
use uuid::Uuid;

pub async fn delete(pool : &Pool<Sqlite>,id : Uuid) -> Result<(),Error> {
  let id = id.to_string();
  match query!(r#"
    DELETE FROM shift_problem WHERE id = $1;
  "#,id).execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn save(pool : &Pool<Sqlite>,problem : ShiftProblem) -> Result<(),Error> {
  let ClientShiftProblem{id,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time} = ClientShiftProblem::new(problem);
  match sqlx::query!(r#"
    INSERT INTO shift_problem(id,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time)
    VALUES($1,$2,$3,$4,$5,$6,$7);
  "#,id,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn update(pool : &Pool<Sqlite>,problem : ShiftProblem) -> Result<(),Error> {
  let ClientShiftProblem{id,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time} = ClientShiftProblem::new(problem);
  match query!(r#"
    UPDATE shift_problem SET
    shift_id        = $2,
    writer_id       = $3,
    maintainer_id   = $4,
    machine_id      = $5,
    begin_time      = $6,
    end_time        = $7
    WHERE id        = $1;
  "#,id,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}
