use rec::model::{spare_part::ClientSparePart, name::Name};
use sqlx::{Pool, Sqlite,Error, query_as};

pub async fn find_all_spare_parts(pool : &Pool<Sqlite>) -> Result<Vec<Name>,Error> {
    match query_as!(Name,r#"
      select * from spare_part;
    "#).fetch_all(pool).await {
      Ok(parts) => Ok(parts),
      Err(err) => Err(err)
    }
}

pub async fn find_spare_parts_by_name(pool : &Pool<Sqlite>,
                        target : &String,canceled : Vec<String>) -> Result<Vec<Name>,Error> {
  let canceled = canceled
    .into_iter()
    .map(|x| format!("'{x}'"))
    .collect::<Vec<String>>()
    .join(",");
  let query = if canceled.is_empty() {
    format!("
    SELECT * FROM spare_part
    WHERE name LIKE '%{target}%' LIMIT 4;")
  } else {
    format!("
    SELECT * FROM spare_part
    WHERE name LIKE '%{target}%' AND name NOT IN ({canceled}) LIMIT 4;")
  };
  match query_as::<_,Name>(&query).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_4_spare_parts(pool : &Pool<Sqlite>,
                          canceled : Vec<String>) -> Result<Vec<Name>,Error> {
  let canceled = canceled
    .into_iter()
    .map(|x| format!("'{x}'"))
    .collect::<Vec<String>>()
    .join(",");
  let query = if canceled.is_empty() {
    format!("
    SELECT * FROM spare_part LIMIT 4;")
  } else {
    format!("
    SELECT * FROM spare_part
    WHERE name NOT IN ({canceled}) LIMIT 4;")
  };
  match query_as::<_,Name>(&query).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_all_spare_parts_names(pool : &Pool<Sqlite>) -> Result<Vec<Name>,Error> {
    match query_as!(Name,r#"
      select id,name from spare_part;
    "#).fetch_all(pool).await {
      Ok(parts) => Ok(parts),
      Err(err) => Err(err)
    }
}

pub async fn find_spare_part_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientSparePart,Error> {
    match query_as!(ClientSparePart,r#"
      SELECT * FROM spare_part WHERE id = $1;
    "#,id).fetch_one(pool).await {
      Ok(problem) => Ok(problem),
      Err(err) => Err(err)
    }
}
