use rec::model::department::{ClientDepartment, Department};
use sqlx::{Pool, Sqlite,Error};

pub async fn find_all_departments(pool : &Pool<Sqlite>) -> Result<Vec<ClientDepartment>,Error> {
    match sqlx::query_as::<_,ClientDepartment>(r#"
      select * from department;
    "#).fetch_all(pool).await {
      Ok(parts) => Ok(parts),
      Err(err) => Err(err)
    }
}

pub async fn find_department_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientDepartment,Error> {
    match sqlx::query_as::<_,ClientDepartment>(r#"
      SELECT * FROM department WHERE id = $1;
    "#).bind(id).fetch_one(pool).await {
      Ok(problem) => Ok(problem),
      Err(err) => Err(err)
    }
}

pub async fn delete_department_by_id(pool : &Pool<Sqlite>,id : String) -> Result<(),Error> {
    match sqlx::query(r#"
      DELETE FROM department WHERE id = $1;
    "#).bind(id).execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}

pub async fn save_department(pool : &Pool<Sqlite>,dep : Department) -> Result<(),Error> {
  let ClientDepartment{id,boss_id,department_id,name} = ClientDepartment::new(dep);
  match sqlx::query(r#"
    INSERT INTO department(id,boss_id,department_id,name)
    VALUES($1,$2,$3,$4);
  "#).bind(id)
  .bind(boss_id)
  .bind(department_id)
  .bind(name)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn update_department(pool : &Pool<Sqlite>,dep : Department) -> Result<(),Error> {
  let ClientDepartment{id,boss_id,department_id,name} = ClientDepartment::new(dep);
    match sqlx::query(r#"
    UPDATE department SET
    boss_id         = $2,
    department_id   = $3,
    name            = $4
    WHERE id        = $1;
    "#).bind(id)
    .bind(boss_id)
    .bind(department_id)
    .bind(name)
    .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}
