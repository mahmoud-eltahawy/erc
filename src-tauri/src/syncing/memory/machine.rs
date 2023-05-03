use rec::model::machine::Machine;
use sqlx::{query, Error, Pool, Sqlite};
use uuid::Uuid;

pub async fn delete(pool: &Pool<Sqlite>, id: Uuid) -> Result<(), Error> {
    let id = id.to_string();
    match query!(
        r#"
    DELETE FROM machine WHERE id = $1;
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

pub async fn save(pool: &Pool<Sqlite>, machine: Machine) -> Result<(), Error> {
    let Machine { id, name } = machine;
    let id = id.to_string();
    match query!(
        r#"
    INSERT INTO machine(id,name)
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

pub async fn update_name(
    pool: &Pool<Sqlite>,
    machine_id: &Uuid,
    name: &String,
) -> Result<(), Error> {
    let machine_id = machine_id.to_string();
    match query!(
        r#"
  UPDATE machine SET
  name = $2
  WHERE id = $1;
  "#,
        machine_id,
        name
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
