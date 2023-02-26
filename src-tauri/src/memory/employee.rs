use rec::model::{employee::ClientEmployee, name::Name};
use sqlx::{Pool, Sqlite,Error, query_as, query};

pub async fn find_all_employees(pool : &Pool<Sqlite>) -> Result<Vec<ClientEmployee>,Error> {
    match query_as!(ClientEmployee,r#"
      select * from employee;
    "#).fetch_all(pool).await {
      Ok(employees) => Ok(employees),
      Err(err) => Err(err)
    }
}

pub async fn find_all_employees_names(pool : &Pool<Sqlite>) -> Result<Vec<Name>,Error> {
    match query!(r#"
      select id,first_name,middle_name,last_name from employee;
    "#).fetch_all(pool).await {
      Ok(records) => Ok(records.into_iter()
                  .map(|r| Name{id:r.id,name: format!("{} {} {}",r.first_name,r.middle_name,r.last_name)}).collect()),
      Err(err) => Err(err)
    }
}

pub async fn find_employee_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientEmployee,Error> {
    match query_as!(ClientEmployee,r#"
      SELECT * FROM employee WHERE id = $1;
    "#,id).fetch_one(pool).await {
      Ok(employee) => Ok(employee),
      Err(err) => Err(err)
    }
}

pub async fn find_employee_by_card(pool : &Pool<Sqlite>,card_id : i64) -> Result<ClientEmployee,Error> {
    match query_as!(ClientEmployee,r#"
      SELECT * FROM employee WHERE card_id = $1;
    "#,card_id).fetch_one(pool).await {
      Ok(employee) => Ok(employee),
      Err(err) => Err(err)
    }
}
