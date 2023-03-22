use rec::model::{employee::ClientEmployee, name::Name};
use sqlx::{Pool, Sqlite,Error, query_as, query};

pub async fn find_all_employees(pool : &Pool<Sqlite>) -> Result<Vec<ClientEmployee>,Error> {
    match query_as!(ClientEmployee,r#"
      select * from employee;
    "#).fetch_all(pool).await {
      Ok(employees) => Ok(employees),
      Err(err) => Err(err)
    }
}

pub async fn find_all_employees_names(pool : &Pool<Sqlite>) -> Result<Vec<Name>,Error> {
    match query_as!(Name,r#"
      SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name FROM employee;
    "#).fetch_all(pool).await {
      Ok(names) => Ok(names),
      Err(err) => Err(err)
    }
}

pub async fn find_employees_by_department_id_except_boss(pool : &Pool<Sqlite>,id : String) -> Result<Vec<Name>,Error> {
    match query_as!(Name,r#"
      SELECT e.id, e.first_name || ' ' || e.middle_name || ' ' ||e.last_name AS name
      FROM employee e WHERE e.department_id = $1
      AND e.id NOT IN (SELECT d.boss_id FROM department d WHERE d.id = $1 AND d.boss_id NOT NULL);
    "#,id).fetch_all(pool).await {
      Ok(names) => Ok(names),
      Err(err) => Err(err)
    }
}

pub async fn find_employee_name_by_id(pool : &Pool<Sqlite>,id : String) -> Result<String,Error> {
    match query!(r#"
      SELECT first_name || ' ' || middle_name || ' ' ||last_name AS name
      FROM employee WHERE id = $1;
    "#,id).fetch_one(pool).await {
      Ok(record) => Ok(record.name),
      Err(err) => Err(err)
    }
}

pub async fn find_employees_by_name(pool : &Pool<Sqlite>,
                        target : &str,canceled : Vec<String>) -> Result<Vec<Name>,Error> {
  let canceled = canceled
    .into_iter()
    .map(|x| format!("'{x}'"))
    .collect::<Vec<String>>()
    .join(",");
  let target = target.split(' ').collect::<Vec<&str>>();
  let cond = match target.len() {
    1 => format!("first_name LIKE '%{}%'",target.first().unwrap()),
    2 => format!("(first_name = '{}' AND middle_name LIKE '%{}%')",
                 target.first().unwrap(),target.get(1).unwrap()),
    3 => format!("(first_name = '{}' AND (middle_name = '{}' AND last_name LIKE '%{}%'))",
                 target.first().unwrap(),target.get(1).unwrap(),target.get(2).unwrap()),
    _ => "id = '0'".to_string()
  };
  let query = if canceled.is_empty() {
    format!("
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name FROM employee
    WHERE {cond} LIMIT 4;")
  } else {
    format!("
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name FROM employee
    WHERE {cond} AND id NOT IN ({canceled}) LIMIT 4;")
  };
  match query_as::<_,Name>(&query).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_4_employees(pool : &Pool<Sqlite>,
                          canceled : Vec<String>) -> Result<Vec<Name>,Error> {
  let canceled = canceled
    .into_iter()
    .map(|x| format!("'{x}'"))
    .collect::<Vec<String>>()
    .join(",");
  let query = if canceled.is_empty() {
    format!("
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name FROM employee LIMIT 4;")
  } else {
    format!("
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name FROM employee
    WHERE id NOT IN ({canceled}) LIMIT 4;")
  };
  match query_as::<_,Name>(&query).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_9_non_admins(pool : &Pool<Sqlite>) -> Result<Vec<Name>,Error> {
  match query_as!(Name,"
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
    FROM employee WHERE position <> 'SUPER_USER' LIMIT 9;").fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_9_non_admins_by_name(pool : &Pool<Sqlite>,target : &str) -> Result<Vec<Name>,Error> {
  let target = target.split(' ').collect::<Vec<&str>>();
  let cond = match target.len() {
    1 => format!("first_name LIKE '%{}%'",target.first().unwrap()),
    2 => format!("(first_name = '{}' AND middle_name LIKE '%{}%')",
                 target.first().unwrap(),target.get(1).unwrap()),
    3 => format!("(first_name = '{}' AND (middle_name = '{}' AND last_name LIKE '%{}%'))",
                 target.first().unwrap(),target.get(1).unwrap(),target.get(2).unwrap()),
    _ => "id = '0'".to_string()
  };
  let query = format!("
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name FROM employee
    WHERE {cond} AND position = 'USER' LIMIT 9;");
  match query_as::<_,Name>(&query).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_admins(pool : &Pool<Sqlite>) -> Result<Vec<Name>,Error> {
  match query_as!(Name,"
    SELECT id, first_name || ' ' || middle_name || ' ' ||last_name AS name
    FROM employee WHERE position = 'SUPER_USER' AND card_id <> 0;").fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_employee_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientEmployee,Error> {
    match query_as!(ClientEmployee,r#"
      SELECT * FROM employee WHERE id = $1;
    "#,id).fetch_one(pool).await {
      Ok(employee) => Ok(employee),
      Err(err) => Err(err)
    }
}

pub async fn find_employee_by_card(pool : &Pool<Sqlite>,card_id : i64) -> Result<ClientEmployee,Error> {
    match query_as!(ClientEmployee,r#"
      SELECT * FROM employee WHERE card_id = $1;
    "#,card_id).fetch_one(pool).await {
      Ok(employee) => Ok(employee),
      Err(err) => Err(err)
    }
}
