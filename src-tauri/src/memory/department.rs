use rec::model::{department::Department, name::Name};
use sqlx::{query, query_as, Error, Pool, Sqlite};

pub async fn find_all_departments(pool: &Pool<Sqlite>) -> Result<Vec<Name<String>>, Error> {
    match query_as!(
        Name,
        r#"
      select id,name from department WHERE name <> 'erc';
    "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(parts) => Ok(parts),
        Err(err) => Err(err),
    }
}

pub async fn find_department_by_id(
    pool: &Pool<Sqlite>,
    id: String,
) -> Result<Department<String>, Error> {
    match query_as!(
        Department,
        r#"
      SELECT * FROM department WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(problem) => Ok(problem),
        Err(err) => Err(err),
    }
}

pub async fn find_department_name_by_id(pool: &Pool<Sqlite>, id: String) -> Result<String, Error> {
    match query!(
        r#"
      SELECT name FROM department WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(record) => Ok(record.name),
        Err(err) => Err(err),
    }
}
