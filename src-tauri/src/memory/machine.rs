use rec::model::{machine::ClientMachine, name::Name};
use sqlx::{Pool, Sqlite,Error, query_as};

pub async fn find_all_machines(pool : &Pool<Sqlite>) -> Result<Vec<ClientMachine>,Error> {
    match query_as!(ClientMachine,r#"
      select * from machine;
    "#).fetch_all(pool).await {
      Ok(machines) => Ok(machines),
      Err(err) => Err(err)
    }
}

pub async fn find_all_machines_names(pool : &Pool<Sqlite>) -> Result<Vec<Name>,Error> {
    match query_as!(Name,r#"
      select id,name from machine;
    "#).fetch_all(pool).await {
      Ok(machines) => Ok(machines),
      Err(err) => Err(err)
    }
}

pub async fn find_machine_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientMachine,Error> {
    match query_as!(ClientMachine,r#"
      SELECT * FROM machine WHERE id = $1;
    "#,id).fetch_one(pool).await {
      Ok(machine) => Ok(machine),
      Err(err) => Err(err)
    }
}
