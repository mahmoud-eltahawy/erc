use rec::model::{spare_part::ClientSparePart, name::Name};
use sqlx::{Pool, Sqlite,Error, query_as};

pub async fn find_all_spare_parts(pool : &Pool<Sqlite>) -> Result<Vec<ClientSparePart>,Error> {
    match query_as!(ClientSparePart,r#"
      select * from spare_part;
    "#).fetch_all(pool).await {
      Ok(parts) => Ok(parts),
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
