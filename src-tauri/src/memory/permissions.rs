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

pub async fn find_department_permissions_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientPermissions,Error> {
    match query_as!(ClientPermissions,r#"
      SELECT p.* FROM permissions p WHERE p.id =
        (SELECT e.id FROM employee e WHERE e.id =
          (SELECT d.boss_id FROM department d WHERE d.id = $1));
    "#,id).fetch_one(pool).await {
      Ok(permissions) => Ok(permissions),
      Err(err) => Err(err)
    }
}

pub async fn find_employee_permissions_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientPermissions,Error> {
    match query_as!(ClientPermissions,r#"
      SELECT p.* FROM permissions p WHERE p.id =
        (SELECT e.id FROM employee e WHERE e.id = $1)
    "#,id).fetch_one(pool).await {
      Ok(permissions) => Ok(permissions),
      Err(err) => Err(err)
    }
}
