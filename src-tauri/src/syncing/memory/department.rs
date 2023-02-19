use rec::model::department::{ClientDepartment, Department};
use sqlx::{Pool, Sqlite,Error, query};
use uuid::Uuid;

pub async fn delete(pool : &Pool<Sqlite>,id : Uuid) -> Result<(),Error> {
  let id = id.to_string();
  match query!(r#"
    DELETE FROM department WHERE id = $1;
  "#,id).execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn save(pool : &Pool<Sqlite>,dep : Department) -> Result<(),Error> {
  let ClientDepartment{id,boss_id,department_id,name} = ClientDepartment::new(dep);
  match query!(r#"
    INSERT INTO department(id,boss_id,department_id,name)
    VALUES($1,$2,$3,$4);
  "#,id,boss_id,department_id,name)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn update(pool : &Pool<Sqlite>,dep : Department) -> Result<(),Error> {
  let ClientDepartment{id,boss_id,department_id,name} = ClientDepartment::new(dep);
    match query!(r#"
    UPDATE department SET
    boss_id         = $2,
    department_id   = $3,
    name            = $4
    WHERE id        = $1;
    "#,id,boss_id,department_id,name)
    .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}
