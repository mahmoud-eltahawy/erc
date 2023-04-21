use sqlx::{query, Error, Pool, Sqlite};

pub async fn last_version(pool: &Pool<Sqlite>) -> Result<i32, Error> {
    match query!(
        r#"
      SELECT the_value as last_version FROM key_value WHERE the_key = 'last_version';
    "#,
    )
    .fetch_one(pool)
    .await
    {
        Ok(rec) => Ok(rec.last_version),
        Err(err) => Err(err),
    }
}

pub async fn save_version(pool: &Pool<Sqlite>, version: u64) -> Result<(), Error> {
    let version_number = version as i64;
    match query!(
        r#"
      UPDATE key_value SET
      the_value = MAX($1,(SELECT the_value from key_value WHERE the_key = 'last_version'))
      WHERE the_key = 'last_version';
    "#,
        version_number
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
