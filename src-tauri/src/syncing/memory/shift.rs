use rec::model::shift::{ClientDbShift, Shift};
use sqlx::{Pool, Sqlite,Error, query};
use uuid::Uuid;


pub async fn delete(pool : &Pool<Sqlite>,id : Uuid) -> Result<(),Error> {
  let id = id.to_string();
  match query!(r#"
    DELETE FROM shift WHERE id = $1;
  "#,id).execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn save(pool : &Pool<Sqlite>,shift : Shift) -> Result<(),Error> {
  let ClientDbShift{id,shift_date,shift_order} = ClientDbShift::new(shift);
  match query!(r#"
    INSERT INTO shift(id,shift_order,shift_date)
    VALUES($1,$2,$3);
  "#,id,shift_date,shift_order)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}
