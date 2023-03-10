use rec::model::machine::{ClientMachine, Machine};
use sqlx::{Pool, Sqlite,Error, query};
use uuid::Uuid;

pub async fn delete(pool : &Pool<Sqlite>,id : Uuid) -> Result<(),Error> {
  let id = id.to_string();
  match query!(r#"
    DELETE FROM machine WHERE id = $1;
  "#,id).execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn save(pool : &Pool<Sqlite>,machine : Machine) -> Result<(),Error> {
  let ClientMachine{id,name} = ClientMachine::new(machine);
  match query!(r#"
    INSERT INTO machine(id,name)
    VALUES($1,$2) ON CONFLICT (id) DO NOTHING;
  "#,id,name)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn update(pool : &Pool<Sqlite>,machine : Machine) -> Result<(),Error> {
  let ClientMachine{id,name} = ClientMachine::new(machine);
    match query!(r#"
    UPDATE machine SET
    name = $2
    WHERE id = $1;
    "#,id,name)
    .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}
