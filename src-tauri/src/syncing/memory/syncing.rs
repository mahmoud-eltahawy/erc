use rec::crud_sync::CudVersion;
use sqlx::{Pool, Sqlite,Error, Row};

pub async fn last_version(pool : &Pool<Sqlite>) -> Result<i64,Error> {
    match sqlx::query(r#"
      SELECT MAX(version_number) FROM cud_version;
    "#).fetch_one(pool).await {
      Ok(v) => Ok(v.get(0)),
      Err(err) => Err(err)
    }
}


pub async fn save_version(pool : &Pool<Sqlite>,version : CudVersion) -> Result<(),Error> {
  let CudVersion{version_number,target_table,target_id,cud,other_target_id} = version;
  let other_target_id = other_target_id.map(|id| id.to_string());
    match sqlx::query(r#"
      INSERT INTO cud_version(version_number,target_table,target_id,cud,other_target_id)
      VALUES($1,$2,$3,$4,$5);
    "#)
     .bind(version_number as i64)
     .bind(target_table as i16)
     .bind(target_id.to_string())
     .bind(cud as i16)
     .bind(other_target_id)
     .execute(pool).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err)
    }
}
