use rec::model::{problem::ClientProblem, name::Name};
use sqlx::{Pool, Sqlite,Error, query_as};

pub async fn find_all_problems(pool : &Pool<Sqlite>) -> Result<Vec<ClientProblem>,Error> {
    match query_as!(ClientProblem,r#"
      select * from problem;
    "#).fetch_all(pool).await {
      Ok(problems) => Ok(problems),
      Err(err) => Err(err)
    }
}

pub async fn find_problem_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientProblem,Error> {
    match query_as!(ClientProblem,r#"
      SELECT * FROM problem WHERE id = $1;
    "#,id).fetch_one(pool).await {
      Ok(problem) => Ok(problem),
      Err(err) => Err(err)
    }
}

pub async fn find_problems_by_department_id(pool : &Pool<Sqlite>,id : String) -> Result<Vec<ClientProblem>,Error> {
    match query_as!(ClientProblem,r#"
      SELECT * FROM problem WHERE department_id = $1;
    "#,id).fetch_all(pool).await {
      Ok(problems) => Ok(problems),
      Err(err) => Err(err)
    }
}

pub async fn find_problems_names_by_department_id(pool : &Pool<Sqlite>,id : String) -> Result<Vec<Name>,Error> {
    match query_as!(Name,r#"
      SELECT id,title as name FROM problem WHERE department_id = $1;
    "#,id).fetch_all(pool).await {
      Ok(problems) => Ok(problems),
      Err(err) => Err(err)
    }
}

pub async fn find_problems_by_writer_id(pool : &Pool<Sqlite>,id : String) -> Result<Vec<ClientProblem>,Error> {
    match query_as!(ClientProblem,r#"
      SELECT * FROM problem WHERE writer_id = $1;
    "#,id).fetch_all(pool).await {
      Ok(problems) => Ok(problems),
      Err(err) => Err(err)
    }
}
