use rec::model::department::Department;
use sqlx::{query, Error, Pool, Sqlite};
use uuid::Uuid;

pub async fn delete(pool: &Pool<Sqlite>, id: Uuid) -> Result<(), Error> {
    let id = id.to_string();
    match query!(
        r#"
    DELETE FROM department WHERE id = $1;
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

pub async fn save(pool: &Pool<Sqlite>, dep: Department<Uuid>) -> Result<(), Error> {
    let Department { id, boss_id, name } = dep.string_to_client();
    match query!(
        r#"
    INSERT INTO department(id,boss_id,name)
    VALUES($1,$2,$3) ON CONFLICT (id) DO NOTHING;
  "#,
        id,
        boss_id,
        name
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn update(pool: &Pool<Sqlite>, dep: Department<Uuid>) -> Result<(), Error> {
    let Department { id, boss_id, name } = dep.string_to_client();
    match query!(
        r#"
  UPDATE department SET
  boss_id         = $2,
  name            = $3
  WHERE id        = $1;
  "#,
        id,
        boss_id,
        name
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
