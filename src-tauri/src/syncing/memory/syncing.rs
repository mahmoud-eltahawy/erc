use sqlx::{query, Pool, Sqlite};
use std::error::Error;

pub async fn last_version(pool: &Pool<Sqlite>) -> Result<(i32, i32), Box<dyn Error>> {
    let cd = query!(
        r#"
      SELECT the_value as last_version FROM key_value WHERE the_key = 'last_cd_version';
    "#,
    )
    .fetch_one(pool)
    .await?;
    let update = query!(
        r#"
      SELECT the_value as last_version FROM key_value WHERE the_key = 'last_update_version';
    "#,
    )
    .fetch_one(pool)
    .await?;
    Ok((cd.last_version, update.last_version))
}

pub async fn save_cd_version(pool: &Pool<Sqlite>, version: u64) -> Result<(), sqlx::Error> {
    let version_number = version as i64;
    match query!(
        r#"
      UPDATE key_value SET
      the_value = MAX($1,(SELECT the_value from key_value WHERE the_key = 'last_cd_version'))
      WHERE the_key = 'last_cd_version';
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

pub async fn save_update_version(pool: &Pool<Sqlite>, version: u64) -> Result<(), sqlx::Error> {
    let version_number = version as i64;
    match query!(
        r#"
      UPDATE key_value SET
      the_value = MAX($1,(SELECT the_value from key_value WHERE the_key = 'last_update_version'))
      WHERE the_key = 'last_update_version';
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
