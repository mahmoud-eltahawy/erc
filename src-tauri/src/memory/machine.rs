use rec::model::{machine::ClientMachine, name::Name};
use sqlx::{Pool, Sqlite,Error, query_as};

pub async fn find_all_machines(pool : &Pool<Sqlite>) -> Result<Vec<Name>,Error> {
    match query_as!(Name,r#"
      select * from machine;
    "#).fetch_all(pool).await {
      Ok(machines) => Ok(machines),
      Err(err) => Err(err)
    }
}

pub async fn find_machines_by_name(pool : &Pool<Sqlite>,
                        target : &str,canceled : Vec<String>) -> Result<Vec<Name>,Error> {
  let canceled = canceled
    .into_iter()
    .map(|x| format!("'{x}'"))
    .collect::<Vec<String>>()
    .join(",");
  let query = if canceled.is_empty() {
    format!("
    SELECT * FROM machine
    WHERE name LIKE '%{target}%' LIMIT 4;")
  } else {
    format!("
    SELECT * FROM machine
    WHERE name LIKE '%{target}%' AND name NOT IN ({canceled}) LIMIT 4;")
  };
  match query_as::<_,Name>(&query).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_4_machines(pool : &Pool<Sqlite>,
                          canceled : Vec<String>) -> Result<Vec<Name>,Error> {
  let canceled = canceled
    .into_iter()
    .map(|x| format!("'{x}'"))
    .collect::<Vec<String>>()
    .join(",");
  let query = if canceled.is_empty() {
    format!("
    SELECT * FROM machine LIMIT 4;")
  } else {
    format!("
    SELECT * FROM machine
    WHERE name NOT IN ({canceled}) LIMIT 4;")
  };
  match query_as::<_,Name>(&query).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
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
