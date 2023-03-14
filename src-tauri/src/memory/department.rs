use rec::model::department::ClientDepartment;
use sqlx::{Pool, Sqlite,Error, query_as, query};

pub async fn find_all_departments(pool : &Pool<Sqlite>) -> Result<Vec<ClientDepartment>,Error> {
    match query_as!(ClientDepartment,r#"
      select * from department;
    "#).fetch_all(pool).await {
      Ok(parts) => Ok(parts),
      Err(err) => Err(err)
    }
}

pub async fn find_department_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientDepartment,Error> {
    match query_as!(ClientDepartment,r#"
      SELECT * FROM department WHERE id = $1;
    "#,id).fetch_one(pool).await {
      Ok(problem) => Ok(problem),
      Err(err) => Err(err)
    }
}

pub async fn find_department_name_by_id(pool : &Pool<Sqlite>,id : String) -> Result<String,Error> {
    match query!(r#"
      SELECT name FROM department WHERE id = $1;
    "#,id).fetch_one(pool).await {
      Ok(record) => Ok(record.name),
      Err(err) => Err(err)
    }
}
