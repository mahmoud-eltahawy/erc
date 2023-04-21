use std::str::FromStr;

use rec::model::name::Name;
use sqlx::{query_as, Error, Pool, Sqlite};
use uuid::Uuid;

pub async fn find_all_machines(pool: &Pool<Sqlite>) -> Result<Vec<Name<String>>, Error> {
    match query_as!(
        Name,
        r#"
      select * from machine;
    "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(machines) => Ok(machines),
        Err(err) => Err(err),
    }
}

pub async fn find_machines_by_name(
    pool: &Pool<Sqlite>,
    target: &str,
    canceled: Vec<String>,
) -> Result<Vec<Name<String>>, Error> {
    let canceled = canceled
        .into_iter()
        .map(|x| format!("'{x}'"))
        .collect::<Vec<String>>()
        .join(",");
    let query = if canceled.is_empty() {
        format!(
            "
    SELECT * FROM machine
    WHERE name LIKE '%{target}%' LIMIT 4;"
        )
    } else {
        format!(
            "
    SELECT * FROM machine
    WHERE name LIKE '%{target}%' AND name NOT IN ({canceled}) LIMIT 4;"
        )
    };
    match query_as::<_, Name<String>>(&query).fetch_all(pool).await {
        Ok(problems) => Ok(problems),
        Err(err) => Err(err),
    }
}

pub async fn find_4_machines(
    pool: &Pool<Sqlite>,
    canceled: Vec<String>,
) -> Result<Vec<Name<String>>, Error> {
    let canceled = canceled
        .into_iter()
        .map(|x| format!("'{x}'"))
        .collect::<Vec<String>>()
        .join(",");
    let query = if canceled.is_empty() {
        format!(
            "
    SELECT * FROM machine LIMIT 4;"
        )
    } else {
        format!(
            "
    SELECT * FROM machine
    WHERE name NOT IN ({canceled}) LIMIT 4;"
        )
    };
    match query_as::<_, Name<String>>(&query).fetch_all(pool).await {
        Ok(problems) => Ok(problems),
        Err(err) => Err(err),
    }
}

pub async fn find_all_machines_names(pool: &Pool<Sqlite>) -> Result<Vec<Name<String>>, Error> {
    match query_as!(
        Name,
        r#"
      select id,name from machine;
    "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(machines) => Ok(machines),
        Err(err) => Err(err),
    }
}

pub async fn find_machine_by_id(pool: &Pool<Sqlite>, id: Uuid) -> Result<Name<Uuid>, Error> {
    let id = id.to_string();
    match query_as!(
        Name,
        r#"
      SELECT * FROM machine WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(machine) => match Uuid::from_str(&machine.id) {
            Ok(id) => Ok(Name {
                id,
                name: machine.name,
            }),
            Err(_) => Err(Error::PoolTimedOut),
        },
        Err(err) => Err(err),
    }
}
