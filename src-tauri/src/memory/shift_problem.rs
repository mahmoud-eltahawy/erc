use rec::model::shift_problem::ClientShiftProblem;
use sqlx::{Pool, Sqlite,Error, query_as};

pub async fn find_shift_shift_problems(pool : &Pool<Sqlite>,
                    shift_id : String,writer_id : String) -> Result<Vec<ClientShiftProblem>,Error> {

    match query_as!(ClientShiftProblem,r#"
        select * from shift_problem
        WHERE shift_id = $1 AND
        writer_id IN (SELECT id FROM employee
        WHERE department_id = (select department_id from employee where id = $2))
    "#,shift_id,writer_id)
    .fetch_all(pool).await {
      Ok(parts) => Ok(parts),
      Err(err) => Err(err)
    }
}

pub async fn find_shift_problem_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientShiftProblem,Error> {
    match query_as!(ClientShiftProblem,r#"
      SELECT * FROM shift_problem WHERE id = $1;
    "#,id).fetch_one(pool).await {
      Ok(problem) => Ok(problem),
      Err(err) => Err(err)
    }
}
