use chrono::NaiveDate;
use rec::{
  model::shift::{ClientDbShift, DateOrder},
  timer::{get_relative_now, get_current_date, get_current_order}};
use sqlx::{Pool, Sqlite,Error, query_as, query};


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

pub async fn find_last_21_shifts(pool : &Pool<Sqlite>) -> Result<Vec<ClientDbShift>,Error> {
  let now = get_relative_now();
  let date = get_current_date(now);
  let date = match date {
    Some(d) => serde_json::json!(d).to_string(),
    None => return Err(Error::PoolClosed)
  };
  match sqlx::query_as!(ClientDbShift,r#"
    SELECT * FROM shift WHERE DATE(substr(shift_date ,2,10)) <= DATE(substr($1 ,2,10)) LIMIT 21;
  "#,date)
  .fetch_all(pool).await {
    Ok(shifts) => Ok(shifts),
    Err(_) => Err(Error::PoolClosed)
  }
}

pub async fn find_shifts_between(pool : &Pool<Sqlite>,
                begin : NaiveDate,end : NaiveDate) -> Result<Vec<ClientDbShift>,Error> {
  let begin = serde_json::json!(begin).to_string();
  let end   = serde_json::json!(end).to_string();
  match sqlx::query_as!(ClientDbShift,r#"
    SELECT * FROM shift WHERE DATE(substr(shift_date ,2,10))
    BETWEEN DATE(substr($1 ,2,10)) AND DATE(substr($2 ,2,10));
  "#,begin,end)
  .fetch_all(pool).await {
    Ok(shifts) => Ok(shifts),
    Err(_) => Err(Error::PoolClosed)
  }
}

pub async fn find_department_shift_id(pool : &Pool<Sqlite>,
                            department_id : &String,shift_id : &String) -> Result<String,Error> {
  let result = query!(r#"SELECT id FROM department_shift
    WHERE department_id = $1 AND shift_id = $2;"#,department_id,shift_id)
  .fetch_one(pool).await?;
  Ok(result.id)
}

pub async fn find_current_department_shift_by_id(pool : &Pool<Sqlite>,
                                        department_id : &String) -> Result<String,Error> {
  let now = get_relative_now();
  let date = get_current_date(now);
  let order = get_current_order(now);
  let order = serde_json::json!(order).to_string();
  if let Some(date) = date {
    let date = serde_json::json!(date).to_string();
    let result = query!(r#"
      SELECT id FROM department_shift
      WHERE department_id = $1 AND shift_id = (
        SELECT id from shift
        WHERE shift_date = $2 AND shift_order = $3
      );"#,department_id,date,order)
    .fetch_one(pool).await?;
    Ok(result.id)
  } else {
    Err(Error::PoolClosed)
  }
}
