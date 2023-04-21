use rec::model::{name::Name, spare_part::SparePart};
use sqlx::{query_as, Error, Pool, Sqlite};

pub async fn find_all_spare_parts(pool: &Pool<Sqlite>) -> Result<Vec<Name<String>>, Error> {
    match query_as!(
        Name,
        r#"
      select * from spare_part;
    "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(parts) => Ok(parts),
        Err(err) => Err(err),
    }
}

pub async fn find_spare_parts_by_name(
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
    SELECT * FROM spare_part
    WHERE name LIKE '%{target}%' LIMIT 4;"
        )
    } else {
        format!(
            "
    SELECT * FROM spare_part
    WHERE name LIKE '%{target}%' AND name NOT IN ({canceled}) LIMIT 4;"
        )
    };
    match query_as::<_, Name<String>>(&query).fetch_all(pool).await {
        Ok(problems) => Ok(problems),
        Err(err) => Err(err),
    }
}

pub async fn find_4_spare_parts(
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
    SELECT * FROM spare_part LIMIT 4;"
        )
    } else {
        format!(
            "
    SELECT * FROM spare_part
    WHERE name NOT IN ({canceled}) LIMIT 4;"
        )
    };
    match query_as::<_, Name<String>>(&query).fetch_all(pool).await {
        Ok(problems) => Ok(problems),
        Err(err) => Err(err),
    }
}

pub async fn find_all_spare_parts_names(pool: &Pool<Sqlite>) -> Result<Vec<Name<String>>, Error> {
    match query_as!(
        Name,
        r#"
      select id,name from spare_part;
    "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(parts) => Ok(parts),
        Err(err) => Err(err),
    }
}

pub async fn find_spare_part_by_id(
    pool: &Pool<Sqlite>,
    id: String,
) -> Result<SparePart<String>, Error> {
    match query_as!(
        SparePart,
        r#"
      SELECT * FROM spare_part WHERE id = $1;
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
