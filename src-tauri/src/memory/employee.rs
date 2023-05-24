use std::str::FromStr;

use itertools::Itertools;
use rec::model::{employee::Employee, name::Name};
use sqlx::{query, Pool, Sqlite};
use uuid::Uuid;

type Error = Box<dyn std::error::Error>;

pub async fn find_all_employees_names(pool: &Pool<Sqlite>) -> Result<Vec<Name>, Error> {
    let records = query!(
        r#"
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name FROM employee;
  "#
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| match Uuid::from_str(&record.id) {
            Ok(id) => Some(Name {
                id,
                name: record.name,
            }),
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_shift_existing_employees_ids(
    pool: &Pool<Sqlite>,
    shift_id: Uuid,
) -> Result<Vec<Uuid>, Error> {
    let shift_id = shift_id.to_string();
    let records = query!(
        r#"
    SELECT e.id FROM employee e WHERE e.id IN
    (SELECT sde.employee_id FROM department_shift_employee sde WHERE sde.department_shift_id = $1)
  "#,
        shift_id
    )
    .fetch_all(pool)
    .await?;

    Ok(records
        .into_iter()
        .flat_map(|record| Uuid::from_str(&record.id))
        .collect_vec())
}

pub async fn does_employee_exist(
    pool: &Pool<Sqlite>,
    shift_id: &Uuid,
    employee_id: &Uuid,
) -> Result<bool, Error> {
    let shift_id = shift_id.to_string();
    let employee_id = employee_id.to_string();
    let record = query!(
        r#"
      SELECT $1 IN (SELECT e.id FROM employee e
      WHERE e.id IN(SELECT sde.employee_id
        FROM department_shift_employee sde WHERE sde.department_shift_id = $2)) AS is_there
  "#,
        employee_id,
        shift_id
    )
    .fetch_one(pool)
    .await?;
    Ok(match record.is_there {
        Some(num) => match num {
            1 => true,
            _ => false,
        },
        None => false,
    })
}

pub async fn find_shift_non_existing_employees_ids(
    pool: &Pool<Sqlite>,
    shift_id: Uuid,
    department_id: Uuid,
) -> Result<Vec<Uuid>, Error> {
    let shift_id = shift_id.to_string();
    let department_id = department_id.to_string();
    let records = query!(
        r#"
    SELECT e.id FROM employee e WHERE e.id NOT IN
    (SELECT sde.employee_id FROM department_shift_employee sde WHERE sde.department_shift_id = $1)
    AND e.department_id = $2
  "#,
        shift_id,
        department_id
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| Uuid::from_str(&record.id))
        .collect_vec())
}

pub async fn find_employees_by_department_id_except_boss(
    pool: &Pool<Sqlite>,
    department_id: &Uuid,
) -> Result<Vec<Name>, Error> {
    let department_id = department_id.to_string();
    let records = query!(
        r#"
    SELECT e.id, e.first_name || ' ' || e.middle_name || ' ' ||e.last_name AS name
    FROM employee e WHERE e.department_id = $1
    AND e.id NOT IN (SELECT d.boss_id FROM department d WHERE d.id = $1 AND d.boss_id NOT NULL);
  "#,
        department_id
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| match Uuid::from_str(&record.id) {
            Ok(id) => Some(Name {
                id,
                name: record.name,
            }),
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_employee_name_by_id(pool: &Pool<Sqlite>, id: Uuid) -> Result<String, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
    SELECT first_name || ' ' || middle_name || ' ' ||last_name AS name
    FROM employee WHERE id = $1;
  "#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(record.name)
}

pub async fn find_employee_department_id_by_id(
    pool: &Pool<Sqlite>,
    id: &Uuid,
) -> Result<Uuid, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
    SELECT department_id
    FROM employee WHERE id = $1;
  "#,
        id
    )
    .fetch_one(pool)
    .await?;
    let result = Uuid::from_str(&record.department_id)?;
    Ok(result)
}

pub async fn find_department_employees_by_name(
    pool: &Pool<Sqlite>,
    target: &str,
    department_id: &Uuid,
) -> Result<Vec<Name>, Error> {
    let target = target.split(' ').collect::<Vec<&str>>();
    let department_id = department_id.to_string();
    match target.len() {
        1 => {
            let name = target.get(0).unwrap().to_string() + "%";
            let records = query!(
                r#"
             SELECT e.id, first_name || ' ' || e.middle_name || ' ' ||e.last_name AS name
             FROM employee e WHERE e.first_name LIKE $1 AND
               (e.department_id = $2 AND e.id NOT IN
                 (SELECT d.boss_id FROM department d WHERE d.id = $1 AND d.boss_id NOT NULL))
           "#,
                name,
                department_id
            )
            .fetch_all(pool)
            .await?;
            Ok(records
                .into_iter()
                .flat_map(|record| match Uuid::from_str(&record.id) {
                    Ok(id) => Some(Name {
                        id,
                        name: record.name,
                    }),
                    Err(_) => None,
                })
                .collect_vec())
        }
        2 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string() + "%";
            let records = query!(
                r#"
             SELECT e.id, e.first_name || ' ' || e.middle_name || ' ' ||e.last_name AS name
             FROM employee e WHERE e.first_name = $1 AND (e.middle_name LIKE $2 AND
               (e.department_id = $3 AND e.id NOT IN
                 (SELECT d.boss_id FROM department d WHERE d.id = $1 AND d.boss_id NOT NULL)))
           "#,
                name0,
                name1,
                department_id
            )
            .fetch_all(pool)
            .await?;
            Ok(records
                .into_iter()
                .flat_map(|record| match Uuid::from_str(&record.id) {
                    Ok(id) => Some(Name {
                        id,
                        name: record.name,
                    }),
                    Err(_) => None,
                })
                .collect_vec())
        }
        3 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string();
            let name2 = target.get(2).unwrap().to_string() + "%";
            let records = query!(
                r#"
             SELECT e.id, e.first_name || ' ' || e.middle_name || ' ' ||e.last_name AS name
             FROM employee e WHERE e.first_name = $1 AND
               (e.middle_name = $2 AND (e.last_name LIKE $3 AND
                 (e.department_id = $4 AND e.id NOT IN
                   (SELECT d.boss_id FROM department d WHERE d.id = $1 AND d.boss_id NOT NULL))))
           "#,
                name0,
                name1,
                name2,
                department_id
            )
            .fetch_all(pool)
            .await?;
            Ok(records
                .into_iter()
                .flat_map(|record| match Uuid::from_str(&record.id) {
                    Ok(id) => Some(Name {
                        id,
                        name: record.name,
                    }),
                    Err(_) => None,
                })
                .collect_vec())
        }
        _ => Ok(vec![]),
    }
}

async fn full_name_search(
    pool: &Pool<Sqlite>,
    target: &str,
    limit: i64,
) -> Result<Vec<Name>, Error> {
    let target = target.split(' ').collect::<Vec<&str>>();

    match target.len() {
        1 => {
            let name = target.get(0).unwrap().to_string() + "%";
            let records = query!(
                r#"
             SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
             FROM employee WHERE first_name LIKE $1 LIMIT $2
           "#,
                name,
                limit
            )
            .fetch_all(pool)
            .await?;
            Ok(records
                .into_iter()
                .flat_map(|record| match Uuid::from_str(&record.id) {
                    Ok(id) => Some(Name {
                        id,
                        name: record.name,
                    }),
                    Err(_) => None,
                })
                .collect_vec())
        }
        2 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string() + "%";
            let records = query!(
                r#"
             SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
             FROM employee WHERE first_name = $1 AND middle_name LIKE $2 LIMIT $3
           "#,
                name0,
                name1,
                limit,
            )
            .fetch_all(pool)
            .await?;
            Ok(records
                .into_iter()
                .flat_map(|record| match Uuid::from_str(&record.id) {
                    Ok(id) => Some(Name {
                        id,
                        name: record.name,
                    }),
                    Err(_) => None,
                })
                .collect_vec())
        }
        3 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string();
            let name2 = target.get(2).unwrap().to_string() + "%";
            let records = query!(
                r#"
             SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
             FROM employee WHERE first_name = $1 AND
             (middle_name = $2 AND last_name LIKE $3) LIMIT $4
           "#,
                name0,
                name1,
                name2,
                limit,
            )
            .fetch_all(pool)
            .await?;
            Ok(records
                .into_iter()
                .flat_map(|record| match Uuid::from_str(&record.id) {
                    Ok(id) => Some(Name {
                        id,
                        name: record.name,
                    }),
                    Err(_) => None,
                })
                .collect_vec())
        }
        _ => Ok(vec![]),
    }
}

pub async fn find_department_8_employees(
    pool: &Pool<Sqlite>,
    department_id: &Uuid,
) -> Result<Vec<Name>, Error> {
    let department_id = department_id.to_string();
    let records = query!(
        "
    SELECT e.id, e.first_name || ' ' || e.middle_name || ' ' ||e.last_name AS name
    FROM employee e WHERE e.department_id = $1
    AND e.id NOT IN (SELECT d.boss_id FROM department d WHERE d.id = $1 AND d.boss_id NOT NULL)
    LIMIT 8;",
        department_id
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| match Uuid::from_str(&record.id) {
            Ok(id) => Some(Name {
                id,
                name: record.name,
            }),
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_9_non_admins_by_name(
    pool: &Pool<Sqlite>,
    target: &str,
) -> Result<Vec<Name>, Error> {
    let target = target.split(' ').collect::<Vec<&str>>();
    match target.len() {
        1 => {
            let name = target.get(0).unwrap().to_string() + "%";
            let records = query!(
                r#"
             SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
             FROM employee WHERE first_name LIKE $1 AND position = 'USER' LIMIT 9
           "#,
                name
            )
            .fetch_all(pool)
            .await?;
            Ok(records
                .into_iter()
                .flat_map(|record| match Uuid::from_str(&record.id) {
                    Ok(id) => Some(Name {
                        id,
                        name: record.name,
                    }),
                    Err(_) => None,
                })
                .collect_vec())
        }
        2 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string() + "%";
            let records = query!(
                r#"
             SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
             FROM employee WHERE first_name = $1 AND
               (middle_name LIKE $2 AND position = 'USER') LIMIT 9
           "#,
                name0,
                name1
            )
            .fetch_all(pool)
            .await?;
            Ok(records
                .into_iter()
                .flat_map(|record| match Uuid::from_str(&record.id) {
                    Ok(id) => Some(Name {
                        id,
                        name: record.name,
                    }),
                    Err(_) => None,
                })
                .collect_vec())
        }
        3 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string();
            let name2 = target.get(2).unwrap().to_string() + "%";
            let records = query!(
                r#"
             SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
             FROM employee WHERE first_name = $1 AND (middle_name = $2 AND
               (last_name LIKE $3 AND position = 'USER')) LIMIT 9
           "#,
                name0,
                name1,
                name2
            )
            .fetch_all(pool)
            .await?;
            Ok(records
                .into_iter()
                .flat_map(|record| match Uuid::from_str(&record.id) {
                    Ok(id) => Some(Name {
                        id,
                        name: record.name,
                    }),
                    Err(_) => None,
                })
                .collect_vec())
        }
        _ => Ok(vec![]),
    }
}

pub async fn find_employees_by_name(
    pool: &Pool<Sqlite>,
    target: &str,
    canceled_ids: Vec<Uuid>,
    limit: i64,
) -> Result<Vec<Name>, Error> {
    let limit = limit + canceled_ids.len() as i64;
    let list = full_name_search(pool, target, limit).await?;

    if canceled_ids.is_empty() {
        Ok(list)
    } else {
        Ok(list
            .into_iter()
            .filter(|n| !canceled_ids.contains(&n.id))
            .collect_vec())
    }
}

pub async fn find_limit_of_employees(
    pool: &Pool<Sqlite>,
    canceled: Vec<Uuid>,
    limit: i64,
) -> Result<Vec<Name>, Error> {
    let limit = limit + canceled.len() as i64;
    let records = query!(
        "
          SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
          FROM employee WHERE card_id <> 0 LIMIT $1;
      ",
        limit
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| match Uuid::from_str(&record.id) {
            Ok(id) => {
                if !canceled.contains(&id) {
                    return Some(Name {
                        id,
                        name: record.name,
                    });
                } else {
                    return None;
                }
            }
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_9_non_admins(pool: &Pool<Sqlite>) -> Result<Vec<Name>, Error> {
    let records = query!(
        "
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
    FROM employee WHERE position = 'USER' LIMIT 9;"
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| match Uuid::from_str(&record.id) {
            Ok(id) => Some(Name {
                id,
                name: record.name,
            }),
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_admins(pool: &Pool<Sqlite>) -> Result<Vec<Name>, Error> {
    let records = query!(
        "
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
    FROM employee WHERE position = 'SUPER_USER' AND card_id <> 0;"
    )
    .fetch_all(pool)
    .await?;
    Ok(records
        .into_iter()
        .flat_map(|record| match Uuid::from_str(&record.id) {
            Ok(id) => Some(Name {
                id,
                name: record.name,
            }),
            Err(_) => None,
        })
        .collect_vec())
}

pub async fn find_employee_by_id(pool: &Pool<Sqlite>, id: &Uuid) -> Result<Employee, Error> {
    let id = id.to_string();
    let record = query!(
        r#"
      SELECT * FROM employee WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await?;
    let id = Uuid::from_str(&record.id)?;
    let department_id = Uuid::from_str(&record.department_id)?;
    Ok(Employee {
        id,
        department_id,
        card_id: record.card_id,
        first_name: record.first_name,
        middle_name: record.middle_name,
        last_name: record.last_name,
        position: record.position,
        password: record.password,
    })
}

pub async fn find_employee_by_card(pool: &Pool<Sqlite>, card_id: i64) -> Result<Employee, Error> {
    let record = query!(
        r#"
      SELECT * FROM employee WHERE card_id = $1;
    "#,
        card_id
    )
    .fetch_one(pool)
    .await?;
    let id = Uuid::from_str(&record.id)?;
    let department_id = Uuid::from_str(&record.department_id)?;
    Ok(Employee {
        id,
        department_id,
        card_id: record.card_id,
        first_name: record.first_name,
        middle_name: record.middle_name,
        last_name: record.last_name,
        position: record.position,
        password: record.password,
    })
}
