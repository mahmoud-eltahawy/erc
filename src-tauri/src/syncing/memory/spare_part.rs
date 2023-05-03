use rec::model::spare_part::SparePart;
use sqlx::{query, Error, Pool, Sqlite};
use uuid::Uuid;

pub async fn delete(pool: &Pool<Sqlite>, id: Uuid) -> Result<(), Error> {
    let id = id.to_string();
    match query!(
        r#"
    DELETE FROM spare_part WHERE id = $1;
  "#,
        id
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn save(pool: &Pool<Sqlite>, part: SparePart) -> Result<(), Error> {
    let SparePart { id, name } = part;
    let id = id.to_string();
    match query!(
        r#"
    INSERT INTO spare_part(id,name)
    VALUES($1,$2) ON CONFLICT (id) DO NOTHING;
  "#,
        id,
        name
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn update_name(pool: &Pool<Sqlite>, part_id: &Uuid, name: &String) -> Result<(), Error> {
    let part_id = part_id.to_string();
    match query!(
        r#"
  UPDATE spare_part SET
  name = $2
  WHERE id = $1;
  "#,
        part_id,
        name
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
