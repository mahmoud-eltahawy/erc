use rec::model::machine::{ClientMachine, Machine};
use sqlx::{Pool, Sqlite,Error};

pub async fn find_all_machines(pool : &Pool<Sqlite>) -> Result<Vec<ClientMachine>,Error> {
    match sqlx::query_as::<_,ClientMachine>(r#"
      select * from machine;
    "#).fetch_all(pool).await {
      Ok(machines) => Ok(machines),
      Err(err) => Err(err)
    }
}

pub async fn find_machine_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientMachine,Error> {
    match sqlx::query_as::<_,ClientMachine>(r#"
      SELECT * FROM machine WHERE id = $1;
    "#).bind(id).fetch_one(pool).await {
      Ok(machine) => Ok(machine),
      Err(err) => Err(err)
    }
}

pub async fn delete_machine_by_id(pool : &Pool<Sqlite>,id : String) -> Result<(),Error> {
    match sqlx::query(r#"
      DELETE FROM machine WHERE id = $1;
    "#).bind(id).execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}

pub async fn save_machine(pool : &Pool<Sqlite>,machine : Machine) -> Result<(),Error> {
  let ClientMachine{id,name} = ClientMachine::new(machine);
  match sqlx::query(r#"
    INSERT INTO machine(id,name)
    VALUES($1,$2);
  "#).bind(id)
  .bind(name)
  .execute(pool).await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn update_machine(pool : &Pool<Sqlite>,machine : Machine) -> Result<(),Error> {
  let ClientMachine{id,name} = ClientMachine::new(machine);
    match sqlx::query(r#"
    UPDATE machine SET
    name = $2
    WHERE id = $1;
    "#).bind(id)
    .bind(name)
    .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}
