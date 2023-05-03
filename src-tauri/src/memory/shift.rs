use std::str::FromStr;

use chrono::NaiveDate;
use itertools::Itertools;
use rec::{
    model::shift::{Shift, ShiftOrder},
    timer::{get_current_date, get_current_order, get_relative_now},
};
use sqlx::{query, Pool, Sqlite};
use uuid::Uuid;

type Error = Box<dyn std::error::Error>;

pub async fn find_all_shifts(pool: &Pool<Sqlite>) -> Result<Vec<Shift>, Error> {
    let records = query!(
        r#"
      select * from shift;
    "#
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|x| {
            match (
                Uuid::from_str(&x.id),
                serde_json::from_str(&x.shift_date),
                ShiftOrder::try_from(x.shift_order),
            ) {
                (Ok(id), Ok(shift_date), Ok(shift_order)) => Some(Shift {
                    id,
                    shift_date,
                    shift_order,
                }),
                _ => None,
            }
        })
        .collect_vec())
}

pub async fn find_shift_by_id(pool: &Pool<Sqlite>, id: Uuid) -> Result<Shift, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
      SELECT * FROM shift WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await?;
    let id = Uuid::from_str(&record.id)?;
    let shift_date = serde_json::from_str(&record.shift_date)?;
    let shift_order = ShiftOrder::try_from(record.shift_order)?;
    Ok(Shift {
        id,
        shift_date,
        shift_order,
    })
}

pub async fn find_shift_by(
    pool: &Pool<Sqlite>,
    date: NaiveDate,
    order: ShiftOrder,
) -> Result<Shift, Error> {
    let date = serde_json::json!(date).to_string();
    let order = order.stringify();
    let record = sqlx::query!(
        r#"
      SELECT * FROM shift WHERE shift_date = $1 AND shift_order =$2;
    "#,
        date,
        order
    )
    .fetch_one(pool)
    .await?;
    let id = Uuid::from_str(&record.id)?;
    let shift_date = serde_json::from_str(&record.shift_date)?;
    let shift_order = ShiftOrder::try_from(record.shift_order)?;
    Ok(Shift {
        id,
        shift_date,
        shift_order,
    })
}

pub async fn find_last_21_shifts(pool: &Pool<Sqlite>) -> Result<Vec<Shift>, Error> {
    let Some(date) = get_current_date(get_relative_now()) else {
        return Err("unvalid date".into());
    };
    let records = sqlx::query!(
        r#"
    SELECT * FROM shift WHERE DATE(substr(shift_date ,2,10)) <= DATE(substr($1 ,2,10)) LIMIT 21;
  "#,
        date
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|x| {
            match (
                Uuid::from_str(&x.id),
                serde_json::from_str(&x.shift_date),
                ShiftOrder::try_from(x.shift_order),
            ) {
                (Ok(id), Ok(shift_date), Ok(shift_order)) => Some(Shift {
                    id,
                    shift_date,
                    shift_order,
                }),
                _ => None,
            }
        })
        .collect_vec())
}

pub async fn find_shifts_between(
    pool: &Pool<Sqlite>,
    begin: NaiveDate,
    end: NaiveDate,
) -> Result<Vec<Shift>, Error> {
    let begin = serde_json::json!(begin).to_string();
    let end = serde_json::json!(end).to_string();
    let records = sqlx::query!(
        r#"
    SELECT * FROM shift WHERE DATE(substr(shift_date ,2,10))
    BETWEEN DATE(substr($1 ,2,10)) AND DATE(substr($2 ,2,10));
  "#,
        begin,
        end
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|x| {
            match (
                Uuid::from_str(&x.id),
                serde_json::from_str(&x.shift_date),
                ShiftOrder::try_from(x.shift_order),
            ) {
                (Ok(id), Ok(shift_date), Ok(shift_order)) => Some(Shift {
                    id,
                    shift_date,
                    shift_order,
                }),
                _ => None,
            }
        })
        .collect_vec())
}

pub async fn find_shifts_after(pool: &Pool<Sqlite>, begin: NaiveDate) -> Result<Vec<Shift>, Error> {
    let begin = serde_json::json!(begin).to_string();
    let records = sqlx::query!(
        r#"
    SELECT * FROM shift WHERE DATE(substr(shift_date ,2,10)) >= DATE(substr($1 ,2,10)) LIMIT 21;
  "#,
        begin
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|x| {
            match (
                Uuid::from_str(&x.id),
                serde_json::from_str(&x.shift_date),
                ShiftOrder::try_from(x.shift_order),
            ) {
                (Ok(id), Ok(shift_date), Ok(shift_order)) => Some(Shift {
                    id,
                    shift_date,
                    shift_order,
                }),
                _ => None,
            }
        })
        .collect_vec())
}

pub async fn find_shifts_before(pool: &Pool<Sqlite>, end: NaiveDate) -> Result<Vec<Shift>, Error> {
    let end = serde_json::json!(end).to_string();
    let records = sqlx::query!(
        r#"
    SELECT * FROM shift WHERE DATE(substr(shift_date ,2,10)) <= DATE(substr($1 ,2,10)) LIMIT 21;
  "#,
        end
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|x| {
            match (
                Uuid::from_str(&x.id),
                serde_json::from_str(&x.shift_date),
                ShiftOrder::try_from(x.shift_order),
            ) {
                (Ok(id), Ok(shift_date), Ok(shift_order)) => Some(Shift {
                    id,
                    shift_date,
                    shift_order,
                }),
                _ => None,
            }
        })
        .collect_vec())
}

pub async fn find_department_shift_id(
    pool: &Pool<Sqlite>,
    department_id: &Uuid,
    shift_id: &Uuid,
) -> Result<Uuid, Error> {
    let department_id = department_id.to_string();
    let shift_id = shift_id.to_string();
    let result = query!(
        r#"SELECT id FROM department_shift
    WHERE department_id = $1 AND shift_id = $2;"#,
        department_id,
        shift_id
    )
    .fetch_one(pool)
    .await?;
    match Uuid::from_str(&result.id) {
        Ok(id) => Ok(id),
        Err(err) => Err(err.into()),
    }
}

pub async fn find_current_department_shift_by_id(
    pool: &Pool<Sqlite>,
    department_id: &Uuid,
) -> Result<Uuid, Error> {
    let department_id = department_id.to_string();
    let now = get_relative_now();
    let Some(date) = get_current_date(now)else {
        return Err("unvalid date".into());
    };
    let order = get_current_order(now).stringify();
    let date = serde_json::json!(date).to_string();
    let result = query!(
        r#"
    SELECT id FROM department_shift
    WHERE department_id = $1 AND shift_id = (
      SELECT id from shift
      WHERE shift_date = $2 AND shift_order = $3
    );"#,
        department_id,
        date,
        order
    )
    .fetch_one(pool)
    .await?;
    let id = Uuid::from_str(&result.id)?;
    Ok(id)
}
