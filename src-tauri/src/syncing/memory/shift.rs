use rec::model::shift::{ClientDbShift, Shift, DepartmentShift, ClientDepartmentShift};
use sqlx::{Pool, Sqlite,Error, query};
use uuid::Uuid;


pub async fn delete_department_shift(pool : &Pool<Sqlite>,id : Uuid) -> Result<(),Error> {
  let id = id.to_string();
  match query!(r#"
    DELETE FROM department_shift WHERE id = $1;
  "#,id).execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn save_department_shift(pool : &Pool<Sqlite>,shift : DepartmentShift) -> Result<(),Error> {
  let ClientDepartmentShift{id,department_id,shift_id} = ClientDepartmentShift::new(shift);
  match query!(r#"
    INSERT INTO department_shift(id,department_id,shift_id)
    VALUES($1,$2,$3);
  "#,id,department_id,shift_id)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn save(pool : &Pool<Sqlite>,shift : Shift) -> Result<(),Error> {
  let ClientDbShift{id,shift_date,shift_order} = ClientDbShift::new(shift);
  match query!(r#"
    INSERT INTO shift(id,shift_date,shift_order)
    VALUES($1,$2,$3);
  "#,id,shift_date,shift_order)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}
