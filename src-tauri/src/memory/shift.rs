use rec::model::shift::{DbShift, Shift, DateOrder};
use sqlx::{Pool, Sqlite,Error};

pub async fn find_all_shifts(pool : &Pool<Sqlite>) -> Result<Vec<DbShift>,Error> {
    match sqlx::query_as::<_,DbShift>(r#"
      select * from shift;
    "#).fetch_all(pool).await {
      Ok(shifts) => Ok(shifts),
      Err(err) => Err(err)
    }
}

pub async fn find_shift_by_id(pool : &Pool<Sqlite>,id : String) -> Result<DbShift,Error> {
    match sqlx::query_as::<_,DbShift>(r#"
      SELECT * FROM shift WHERE id = $1;
    "#).bind(id).fetch_one(pool).await {
      Ok(employee) => Ok(employee),
      Err(err) => Err(err)
    }
}

pub async fn find_shift_by(pool : &Pool<Sqlite>,date_and_order : DateOrder) -> Option<DbShift> {
    let DateOrder{date,order} = date_and_order;
    match sqlx::query_as::<_,DbShift>(r#"
      SELECT * FROM shift WHERE shift_date = $1 AND shift_order =$2;
    "#)
    .bind(date)
    .bind(order)
    .fetch_one(pool).await {
      Ok(employee) => Some(employee),
      Err(_) => None
    }
}

pub async fn delete_shift_by_id(pool : &Pool<Sqlite>,id : String) -> Result<(),Error> {
    match sqlx::query(r#"
      DELETE FROM shift WHERE id = $1;
    "#).bind(id).execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}

pub async fn save_shift(pool : &Pool<Sqlite>,shift : Shift) -> Result<(),Error> {
  let Shift{id,shift_date,shift_order} = shift;
    match sqlx::query(r#"
      INSERT INTO shift(id,shift_order,shift_date)
      VALUES($1,$2,$3);
    "#).bind(id.to_string())
    .bind(shift_date)
    .bind(shift_order as i16)
    .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}
