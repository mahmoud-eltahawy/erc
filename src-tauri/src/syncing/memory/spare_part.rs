use rec::model::spare_part::{ClientSparePart, SparePart};
use sqlx::{Pool, Sqlite,Error, query};
use uuid::Uuid;

pub async fn delete(pool : &Pool<Sqlite>,id : Uuid) -> Result<(),Error> {
  let id = id.to_string();
  match query!(r#"
    DELETE FROM spare_part WHERE id = $1;
  "#,id).execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn save(pool : &Pool<Sqlite>,part : SparePart) -> Result<(),Error> {
  let ClientSparePart{id,name} = ClientSparePart::new(part);
  match query!(r#"
    INSERT INTO spare_part(id,name)
    VALUES($1,$2) ON CONFLICT (id) DO NOTHING;
  "#,id,name)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn update(pool : &Pool<Sqlite>,part : SparePart) -> Result<(),Error> {
  let ClientSparePart{id,name} = ClientSparePart::new(part);
    match query!(r#"
    UPDATE spare_part SET
    name = $2
    WHERE id = $1;
    "#,id,name)
    .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}
