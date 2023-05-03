use itertools::Itertools;
use rec::model::permissions::PermissionName;
use sqlx::{query, Pool, Sqlite};
use uuid::Uuid;

type Error = Box<dyn std::error::Error>;

pub async fn find_permissions_by_id(
    pool: &Pool<Sqlite>,
    id: Uuid,
) -> Result<Vec<PermissionName>, Error> {
    let id = id.to_string();
    let records = query!(
        r#"
      SELECT permission FROM permissions WHERE employee_id = $1;
    "#,
        id
    )
    .fetch_all(pool)
    .await?;

    Ok(records
        .into_iter()
        .flat_map(|x| PermissionName::try_from(x.permission))
        .collect_vec())
}
