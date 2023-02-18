use rec::model::spare_part::{ClientSparePart, SparePart};
use sqlx::{Pool, Sqlite,Error};

pub async fn find_all_spare_parts(pool : &Pool<Sqlite>) -> Result<Vec<ClientSparePart>,Error> {
    match sqlx::query_as::<_,ClientSparePart>(r#"
      select * from spare_part;
    "#).fetch_all(pool).await {
      Ok(parts) => Ok(parts),
      Err(err) => Err(err)
    }
}

pub async fn find_spare_part_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientSparePart,Error> {
    match sqlx::query_as::<_,ClientSparePart>(r#"
      SELECT * FROM spare_part WHERE id = $1;
    "#).bind(id).fetch_one(pool).await {
      Ok(problem) => Ok(problem),
      Err(err) => Err(err)
    }
}

pub async fn delete_spare_part_by_id(pool : &Pool<Sqlite>,id : String) -> Result<(),Error> {
    match sqlx::query(r#"
      DELETE FROM spare_part WHERE id = $1;
    "#).bind(id).execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}

pub async fn save_spare_part(pool : &Pool<Sqlite>,part : SparePart) -> Result<(),Error> {
  let ClientSparePart{id,name} = ClientSparePart::new(part);
  match sqlx::query(r#"
    INSERT INTO spare_part(id,name)
    VALUES($1,$2);
  "#).bind(id)
  .bind(name)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn update_spare_part(pool : &Pool<Sqlite>,part : SparePart) -> Result<(),Error> {
  let ClientSparePart{id,name} = ClientSparePart::new(part);
    match sqlx::query(r#"
    UPDATE spare_part SET
    SET name = $2
    WHERE id = $1;
    "#).bind(id)
    .bind(name)
    .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}
