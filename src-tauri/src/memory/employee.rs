use rec::model::employee::{ClientEmployee, Employee};
use sqlx::{Pool, Sqlite,Error};

pub async fn find_all_employees(pool : &Pool<Sqlite>) -> Result<Vec<ClientEmployee>,Error> {
    match sqlx::query_as::<_,ClientEmployee>(r#"
      select * from employee;
    "#).fetch_all(pool).await {
      Ok(employees) => Ok(employees),
      Err(err) => Err(err)
    }
}

pub async fn find_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientEmployee,Error> {
    match sqlx::query_as::<_,ClientEmployee>(r#"
      SELECT * FROM employee WHERE id = $1;
    "#).bind(id).fetch_one(pool).await {
      Ok(employee) => Ok(employee),
      Err(err) => Err(err)
    }
}

pub async fn delete_employee_by_id(pool : &Pool<Sqlite>,id : String) -> Result<(),Error> {
    match sqlx::query(r#"
      DELETE FROM employee WHERE id = $1;
    "#).bind(id).execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}

pub async fn find_employee_by_card(pool : &Pool<Sqlite>,card_id : i16) -> Result<ClientEmployee,Error> {
    match sqlx::query_as::<_,ClientEmployee>(r#"
      SELECT * FROM employee WHERE card_id = $1;
    "#).bind(card_id).fetch_one(pool).await {
      Ok(employee) => Ok(employee),
      Err(err) => Err(err)
    }
}

pub async fn save_employee(pool : &Pool<Sqlite>,employee : Employee) -> Result<(),Error> {
  let Employee{id,card_id,department_id,first_name,last_name,middle_name,position,password} = employee;
    match sqlx::query(r#"
      INSERT INTO employee(id,card_id,department_id,first_name,last_name,middle_name,position,password)
      VALUES($1,$2,$3,$4,$5,$6,$7,$8);
    "#).bind(id.to_string())
    .bind(card_id)
    .bind(department_id.to_string())
    .bind(first_name)
    .bind(last_name)
    .bind(middle_name)
    .bind(position)
    .bind(password)
    .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}

pub async fn update_employee(pool : &Pool<Sqlite>,employee : Employee) -> Result<(),Error> {
  let Employee{id,card_id,department_id,first_name,last_name,middle_name,position,password} = employee;
    match sqlx::query(r#"
    UPDATE employee SET
    department_id = $2,
    position      = $3,
    first_name    = $4,
    middle_name   = $5,
    last_name     = $6,
    card_id       = $7,
    password      = $8
    WHERE id = $1;
    "#).bind(id.to_string())
    .bind(department_id.to_string())
    .bind(position)
    .bind(first_name)
    .bind(middle_name)
    .bind(last_name)
    .bind(card_id)
    .bind(password)
    .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}
