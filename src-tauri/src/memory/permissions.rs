use rec::model::permissions::ClientPermissions;
use sqlx::{query_as,Error, Pool, Sqlite};

pub async fn find_permissions_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientPermissions,Error> {
    match query_as!(ClientPermissions,r#"
      SELECT * FROM permissions WHERE id = $1;
    "#,id).fetch_one(pool).await {
      Ok(permissions) => Ok(permissions),
      Err(err) => Err(err)
    }
}
