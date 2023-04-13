use itertools::Itertools;
use rec::model::{employee::Employee, name::Name};
use sqlx::{query, query_as, Error, Pool, Sqlite};
use uuid::Uuid;

//TODO boilerplate code in search functions could be eliminated with macros

pub async fn find_all_employees_names(pool: &Pool<Sqlite>) -> Result<Vec<Name>, Error> {
    match query_as!(
        Name,
        r#"
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name FROM employee;
  "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(names) => Ok(names),
        Err(err) => Err(err),
    }
}

pub async fn find_shift_existing_employees_names(
    pool: &Pool<Sqlite>,
    shift_id: Uuid,
) -> Result<Vec<Name>, Error> {
    let shift_id = shift_id.to_string();
    match query_as!(
        Name,
        r#"
    SELECT e.id, e.first_name || ' ' || e.middle_name || ' ' ||e.last_name AS name
    FROM employee e WHERE e.id IN
    (SELECT sde.employee_id FROM department_shift_employee sde WHERE sde.department_shift_id = $1)
  "#,
        shift_id
    )
    .fetch_all(pool)
    .await
    {
        Ok(names) => Ok(names),
        Err(err) => Err(err),
    }
}

pub async fn does_employee_exist(
    pool: &Pool<Sqlite>,
    shift_id: &Uuid,
    employee_id: &Uuid,
) -> Result<bool, Error> {
    let shift_id = shift_id.to_string();
    let employee_id = employee_id.to_string();
    match query!(
        r#"
      SELECT $1 IN (SELECT e.id FROM employee e
      WHERE e.id IN(SELECT sde.employee_id
        FROM department_shift_employee sde WHERE sde.department_shift_id = $2)) AS is_there
  "#,
        employee_id,
        shift_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(record) => match record.is_there {
            Some(num) => match num {
                1 => Ok(true),
                _ => Ok(false),
            },
            None => Ok(false),
        },
        Err(err) => Err(err),
    }
}

pub async fn find_shift_non_existing_employees_names(
    pool: &Pool<Sqlite>,
    shift_id: Uuid,
    department_id: Uuid,
) -> Result<Vec<Name>, Error> {
    let shift_id = shift_id.to_string();
    let department_id = department_id.to_string();
    match query_as!(
        Name,
        r#"
    SELECT e.id, e.first_name || ' ' || e.middle_name || ' ' ||e.last_name AS name
    FROM employee e WHERE e.id NOT IN
    (SELECT sde.employee_id FROM department_shift_employee sde WHERE sde.department_shift_id = $1)
    AND e.department_id = $2
  "#,
        shift_id,
        department_id
    )
    .fetch_all(pool)
    .await
    {
        Ok(names) => Ok(names),
        Err(err) => Err(err),
    }
}

pub async fn find_employees_by_department_id_except_boss(
    pool: &Pool<Sqlite>,
    department_id: &Uuid,
) -> Result<Vec<Name>, Error> {
    let department_id = department_id.to_string();
    match query_as!(
        Name,
        r#"
    SELECT e.id, e.first_name || ' ' || e.middle_name || ' ' ||e.last_name AS name
    FROM employee e WHERE e.department_id = $1
    AND e.id NOT IN (SELECT d.boss_id FROM department d WHERE d.id = $1 AND d.boss_id NOT NULL);
  "#,
        department_id
    )
    .fetch_all(pool)
    .await
    {
        Ok(names) => Ok(names),
        Err(err) => Err(err),
    }
}

pub async fn find_employee_name_by_id(pool: &Pool<Sqlite>, id: String) -> Result<String, Error> {
    match query!(
        r#"
    SELECT first_name || ' ' || middle_name || ' ' ||last_name AS name
    FROM employee WHERE id = $1;
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
            match query_as!(
                Name,
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
            .await
            {
                Ok(emps) => Ok(emps),
                Err(err) => Err(err.into()),
            }
        }
        2 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string() + "%";
            match query_as!(
                Name,
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
            .await
            {
                Ok(emps) => Ok(emps),
                Err(err) => Err(err.into()),
            }
        }
        3 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string();
            let name2 = target.get(2).unwrap().to_string() + "%";
            match query_as!(
                Name,
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
            .await
            {
                Ok(emps) => Ok(emps),
                Err(err) => Err(err.into()),
            }
        }
        _ => Ok(vec![]),
    }
}

async fn full_name_search(pool: &Pool<Sqlite>, target: &str) -> Result<Vec<Name>, Error> {
    let target = target.split(' ').collect::<Vec<&str>>();

    match target.len() {
        1 => {
            let name = target.get(0).unwrap().to_string() + "%";
            match query_as!(
                Name,
                r#"
             SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
             FROM employee WHERE first_name LIKE $1
           "#,
                name
            )
            .fetch_all(pool)
            .await
            {
                Ok(emps) => Ok(emps),
                Err(err) => Err(err.into()),
            }
        }
        2 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string() + "%";
            match query_as!(
                Name,
                r#"
             SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
             FROM employee WHERE first_name = $1 AND middle_name LIKE $2
           "#,
                name0,
                name1
            )
            .fetch_all(pool)
            .await
            {
                Ok(emps) => Ok(emps),
                Err(err) => Err(err.into()),
            }
        }
        3 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string();
            let name2 = target.get(2).unwrap().to_string() + "%";
            match query_as!(
                Name,
                r#"
             SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
             FROM employee WHERE first_name = $1 AND (middle_name = $2 AND last_name LIKE $3)
           "#,
                name0,
                name1,
                name2
            )
            .fetch_all(pool)
            .await
            {
                Ok(emps) => Ok(emps),
                Err(err) => Err(err.into()),
            }
        }
        _ => Ok(vec![]),
    }
}

pub async fn find_department_8_employees(
    pool: &Pool<Sqlite>,
    department_id: &Uuid,
) -> Result<Vec<Name>, Error> {
    let department_id = department_id.to_string();
    let query = format!(
        "
    SELECT e.id, e.first_name || ' ' || e.middle_name || ' ' ||e.last_name AS name
    FROM employee e WHERE e.department_id = $1
    AND e.id NOT IN (SELECT d.boss_id FROM department d WHERE d.id = $1 AND d.boss_id NOT NULL)
    LIMIT 8;"
    );
    match query_as::<_, Name>(&query)
        .bind(department_id)
        .fetch_all(pool)
        .await
    {
        Ok(problems) => Ok(problems),
        Err(err) => Err(err),
    }
}

pub async fn find_9_non_admins_by_name(
    pool: &Pool<Sqlite>,
    target: &str,
) -> Result<Vec<Name>, Error> {
    let target = target.split(' ').collect::<Vec<&str>>();
    match target.len() {
        1 => {
            let name = target.get(0).unwrap().to_string() + "%";
            match query_as!(
                Name,
                r#"
             SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
             FROM employee WHERE first_name LIKE $1 AND position = 'USER' LIMIT 9
           "#,
                name
            )
            .fetch_all(pool)
            .await
            {
                Ok(emps) => Ok(emps),
                Err(err) => Err(err.into()),
            }
        }
        2 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string() + "%";
            match query_as!(
                Name,
                r#"
             SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
             FROM employee WHERE first_name = $1 AND
               (middle_name LIKE $2 AND position = 'USER') LIMIT 9
           "#,
                name0,
                name1
            )
            .fetch_all(pool)
            .await
            {
                Ok(emps) => Ok(emps),
                Err(err) => Err(err.into()),
            }
        }
        3 => {
            let name0 = target.get(0).unwrap().to_string();
            let name1 = target.get(1).unwrap().to_string();
            let name2 = target.get(2).unwrap().to_string() + "%";
            match query_as!(
                Name,
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
            .await
            {
                Ok(emps) => Ok(emps),
                Err(err) => Err(err.into()),
            }
        }
        _ => Ok(vec![]),
    }
}

pub async fn find_employees_by_name(
    pool: &Pool<Sqlite>,
    target: &str,
    canceled_ids: Vec<String>,
) -> Result<Vec<Name>, Error> {
    let list = match full_name_search(pool, target).await {
        Ok(list) => list,
        Err(err) => return Err(err.into()),
    };

    if canceled_ids.is_empty() {
        Ok(list)
    } else {
        Ok(list
            .into_iter()
            .filter(|n| !canceled_ids.contains(&n.id))
            .collect_vec())
    }
}

pub async fn find_4_employees(
    pool: &Pool<Sqlite>,
    canceled: Vec<String>,
) -> Result<Vec<Name>, Error> {
    let canceled = canceled
        .into_iter()
        .map(|x| format!("'{x}'"))
        .collect::<Vec<String>>()
        .join(",");
    let query = if canceled.is_empty() {
        format!(
            "
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name FROM employee LIMIT 4;"
        )
    } else {
        format!(
            "
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name FROM employee
    WHERE id NOT IN ({canceled}) LIMIT 4;"
        )
    };
    match query_as::<_, Name>(&query).fetch_all(pool).await {
        Ok(problems) => Ok(problems),
        Err(err) => Err(err),
    }
}

pub async fn find_9_non_admins(pool: &Pool<Sqlite>) -> Result<Vec<Name>, Error> {
    match query_as!(
        Name,
        "
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
    FROM employee WHERE position = 'USER' LIMIT 9;"
    )
    .fetch_all(pool)
    .await
    {
        Ok(problems) => Ok(problems),
        Err(err) => Err(err),
    }
}

pub async fn find_admins(pool: &Pool<Sqlite>) -> Result<Vec<Name>, Error> {
    match query_as!(
        Name,
        "
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
    FROM employee WHERE position = 'SUPER_USER' AND card_id <> 0;"
    )
    .fetch_all(pool)
    .await
    {
        Ok(problems) => Ok(problems),
        Err(err) => Err(err),
    }
}

pub async fn find_employee_by_id(
    pool: &Pool<Sqlite>,
    id: String,
) -> Result<Employee<String>, Error> {
    match query_as!(
        Employee,
        r#"
      SELECT * FROM employee WHERE id = $1;
    "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(employee) => Ok(employee),
        Err(err) => Err(err),
    }
}

pub async fn find_employee_by_card(
    pool: &Pool<Sqlite>,
    card_id: i64,
) -> Result<Employee<String>, Error> {
    match query_as!(
        Employee,
        r#"
      SELECT * FROM employee WHERE card_id = $1;
    "#,
        card_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(employee) => Ok(employee),
        Err(err) => Err(err),
    }
}
