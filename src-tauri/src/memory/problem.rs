use rec::model::{problem::ClientProblem, name::Name};
use sqlx::{Pool, Sqlite,Error, query_as};

pub async fn find_all_problems(pool : &Pool<Sqlite>) -> Result<Vec<ClientProblem>,Error> {
  match query_as!(ClientProblem,r#"
    select * from problem;
  "#).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_department_all_problems(pool : &Pool<Sqlite>,
                                  department_id : String) -> Result<Vec<Name>,Error> {
  match query_as!(Name,r#"
    select id,title as name from problem WHERE department_id = $1;
  "#,department_id).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_problem_by_id(pool : &Pool<Sqlite>,id : String) -> Result<ClientProblem,Error> {
  match query_as!(ClientProblem,r#"
    SELECT * FROM problem WHERE id = $1;
  "#,id).fetch_one(pool).await {
    Ok(problem) => Ok(problem),
    Err(err) => Err(err)
  }
}

pub async fn find_problems_by_department_id(pool : &Pool<Sqlite>,id : String) -> Result<Vec<Name>,Error> {
  match query_as!(Name,r#"
    SELECT id , title as name FROM problem WHERE department_id = $1 LIMIT 7;
  "#,id).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_department_problems_by_name(pool : &Pool<Sqlite>,
                        department_id : String,target : &String,canceled : Vec<String>) -> Result<Vec<Name>,Error> {
  let canceled = canceled
    .into_iter()
    .map(|x| format!("'{x}'"))
    .collect::<Vec<String>>()
    .join(",");
  let query = if canceled.is_empty() {
    format!("
    SELECT id,title as name FROM problem
    WHERE department_id = '{department_id}'
    AND title LIKE '%{target}%'
    LIMIT 4;")
  } else {
    format!("
    SELECT id,title as name FROM problem
    WHERE department_id = '{department_id}'
    AND (title LIKE '%{target}%' AND title NOT IN ({canceled}))
    LIMIT 4;")
  };
  match query_as::<_,Name>(&query).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_department_full_problems_by_name(pool : &Pool<Sqlite>,
                        department_id : String,target : &String) -> Result<Vec<Name>,Error> {
  let target = format!("%{target}%");
  match query_as!(Name,r#"
    SELECT id ,title as name FROM problem
    WHERE department_id = $1
    AND title LIKE $2
    LIMIT 8;"#,department_id,target).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_department_4_problems(pool : &Pool<Sqlite>,
                          department_id : String,canceled : Vec<String>) -> Result<Vec<Name>,Error> {
  let canceled = canceled
    .into_iter()
    .map(|x| format!("'{x}'"))
    .collect::<Vec<String>>()
    .join(",");
  let query = if canceled.is_empty() {
    format!("
    SELECT id,title as name FROM problem
    WHERE department_id = '{department_id}'
    LIMIT 4;")
  } else {
    format!("
    SELECT id,title as name FROM problem
    WHERE department_id = '{department_id}'
    AND title NOT IN ({canceled})
    LIMIT 4;")
  };
  match query_as::<_,Name>(&query).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}

pub async fn find_problems_by_writer_id(pool : &Pool<Sqlite>,id : String) -> Result<Vec<ClientProblem>,Error> {
  match query_as!(ClientProblem,r#"
    SELECT * FROM problem WHERE writer_id = $1;
  "#,id).fetch_all(pool).await {
    Ok(problems) => Ok(problems),
    Err(err) => Err(err)
  }
}
