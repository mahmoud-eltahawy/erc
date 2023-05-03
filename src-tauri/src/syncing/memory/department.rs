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

pub async fn save(pool: &Pool<Sqlite>, dep: Department) -> Result<(), Error> {
    let Department { id, boss_id, name } = dep;
    let id = id.to_string();
    let boss_id = boss_id.map(|id| id.to_string());
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

pub async fn update_name(pool: &Pool<Sqlite>, dep_id: &Uuid, name: &String) -> Result<(), Error> {
    let dep_id = dep_id.to_string();
    match query!(
        r#"
  UPDATE department SET
  name            = $2
  WHERE id        = $1;
  "#,
        dep_id,
        name
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn set_boss(pool: &Pool<Sqlite>, dep_id: &Uuid, boss_id: &Uuid) -> Result<(), Error> {
    let dep_id = dep_id.to_string();
    let boss_id = boss_id.to_string();
    match query!(
        r#"
  UPDATE department SET
  boss_id         = $2
  WHERE id        = $1;
  "#,
        dep_id,
        boss_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn change_boss(pool: &Pool<Sqlite>, boss_id: &Uuid) -> Result<(), Error> {
    let boss_id = boss_id.to_string();
    match query!(
        r#"
  UPDATE department SET
  boss_id  = $1
  WHERE id = (SELECT department_id FROM employee WHERE id = $1);
  "#,
        boss_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
