use rec::model::shift::{ClientDbShift, DateOrder};
use sqlx::{Pool, Sqlite,Error, query_as};


pub async fn find_all_shifts(pool : &Pool<Sqlite>) -> Result<Vec<ClientDbShift>,Error> {
    match query_as!(ClientDbShift,r#"
      select * from shift;
    "#).fetch_all(pool).await {
      Ok(shifts) => Ok(shifts),
      Err(err) => Err(err)
    }
}

pub async fn find_shift_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientDbShift,Error> {
    match query_as!(ClientDbShift,r#"
      SELECT * FROM shift WHERE id = $1;
    "#,id).fetch_one(pool).await {
      Ok(employee) => Ok(employee),
      Err(err) => Err(err)
    }
}

pub async fn find_shift_by(pool : &Pool<Sqlite>,date_and_order : DateOrder) -> Option<ClientDbShift> {
    let DateOrder{date,order} = date_and_order;
    match sqlx::query_as!(ClientDbShift,r#"
      SELECT * FROM shift WHERE shift_date = $1 AND shift_order =$2;
    "#,date,order)
    .fetch_one(pool).await {
      Ok(employee) => Some(employee),
      Err(_) => None
    }
}
