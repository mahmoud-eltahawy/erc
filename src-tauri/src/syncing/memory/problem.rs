use rec::model::problem::{ClientProblem, Probelm};
use sqlx::{Pool, Sqlite,Error, query};
use uuid::Uuid;

pub async fn delete(pool : &Pool<Sqlite>,id : Uuid) -> Result<(),Error> {
  let id = id.to_string();
  match query!(r#"
    DELETE FROM problem WHERE id = $1;
  "#,id).execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn save(pool : &Pool<Sqlite>,problem : Probelm) -> Result<(),Error> {
  let ClientProblem{id,writer_id,department_id,title,description} = ClientProblem::new(problem);
  match query!(r#"
    INSERT INTO problem(id,writer_id,department_id,title,description)
    VALUES($1,$2,$3,$4,$5);
  "#,id,writer_id,department_id,title,description)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn update(pool : &Pool<Sqlite>,problem : Probelm) -> Result<(),Error> {
  let ClientProblem{id,writer_id,department_id,title,description} = ClientProblem::new(problem);
    match query!(r#"
    UPDATE problem SET
    writer_id     = $2,
    department_id = $3,
    title         = $4,
    description   = $5
    WHERE id = $1;
    "#,id,writer_id,department_id,title,description)
    .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}
