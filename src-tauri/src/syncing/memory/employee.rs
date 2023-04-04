use rec::model::employee::Employee;
use sqlx::{Pool, Sqlite,Error, query};
use uuid::Uuid;

pub async fn delete(pool : &Pool<Sqlite>,id : Uuid) -> Result<(),Error> {
  let id = id.to_string();
  match query!(r#"
    DELETE FROM employee WHERE id = $1;
  "#,id).execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn save(pool : &Pool<Sqlite>,employee : Employee<Uuid>) -> Result<(),Error> {
  let Employee{id,card_id,department_id,first_name,
                   last_name,middle_name,position,password} = employee.string_to_client();
  match query!(r#"
    INSERT INTO employee(id,card_id,department_id,first_name,last_name,middle_name,position,password)
    VALUES($1,$2,$3,$4,$5,$6,$7,$8) ON CONFLICT (id) DO NOTHING;
  "#,id,card_id,department_id,first_name,last_name,middle_name,position,password)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn update(pool : &Pool<Sqlite>,employee : Employee<Uuid>) -> Result<(),Error> {
  let Employee{id,card_id,department_id,first_name,
                   last_name,middle_name,position,password} = employee.string_to_client();
  match query!(r#"
  UPDATE employee SET
  card_id       = $2,
  department_id = $3,
  position      = $4,
  first_name    = $5,
  last_name     = $6,
  middle_name   = $7,
  password      = $8
  WHERE id = $1;
  "#,id,card_id,department_id,position,first_name,last_name,middle_name,password)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}
