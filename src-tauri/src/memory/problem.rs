use rec::model::problem::{ClientProblem, Probelm};
use sqlx::{Pool, Sqlite,Error};

pub async fn find_all_problems(pool : &Pool<Sqlite>) -> Result<Vec<ClientProblem>,Error> {
    match sqlx::query_as::<_,ClientProblem>(r#"
      select * from problem;
    "#).fetch_all(pool).await {
      Ok(problems) => Ok(problems),
      Err(err) => Err(err)
    }
}

pub async fn find_problem_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientProblem,Error> {
    match sqlx::query_as::<_,ClientProblem>(r#"
      SELECT * FROM problem WHERE id = $1;
    "#).bind(id).fetch_one(pool).await {
      Ok(problem) => Ok(problem),
      Err(err) => Err(err)
    }
}

pub async fn find_problems_by_department_id(pool : &Pool<Sqlite>,id : String) -> Result<Vec<ClientProblem>,Error> {
    match sqlx::query_as::<_,ClientProblem>(r#"
      SELECT * FROM problem WHERE department_id = $1;
    "#).bind(id).fetch_all(pool).await {
      Ok(problems) => Ok(problems),
      Err(err) => Err(err)
    }
}

pub async fn find_problems_by_writer_id(pool : &Pool<Sqlite>,id : String) -> Result<Vec<ClientProblem>,Error> {
    match sqlx::query_as::<_,ClientProblem>(r#"
      SELECT * FROM problem WHERE writer_id = $1;
    "#).bind(id).fetch_all(pool).await {
      Ok(problems) => Ok(problems),
      Err(err) => Err(err)
    }
}

pub async fn delete_problem_by_id(pool : &Pool<Sqlite>,id : String) -> Result<(),Error> {
    match sqlx::query(r#"
      DELETE FROM problem WHERE id = $1;
    "#).bind(id).execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}

pub async fn save_problem(pool : &Pool<Sqlite>,problem : Probelm) -> Result<(),Error> {
  let ClientProblem{id,writer_id,department_id,title,description} = ClientProblem::new(problem);
  match sqlx::query(r#"
    INSERT INTO problem(id,writer_id,department_id,title,description)
    VALUES($1,$2,$3,$4,$5);
  "#).bind(id.to_string())
  .bind(writer_id)
  .bind(department_id)
  .bind(title)
  .bind(description)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn update_problem(pool : &Pool<Sqlite>,problem : Probelm) -> Result<(),Error> {
  let ClientProblem{id,writer_id,department_id,title,description} = ClientProblem::new(problem);
    match sqlx::query(r#"
    UPDATE employee SET
    writer_id     = $2,
    department_id = $3,
    title         = $4,
    description   = $5
    WHERE id = $1;
    "#).bind(id.to_string())
    .bind(writer_id)
    .bind(department_id)
    .bind(title)
    .bind(description)
    .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}
