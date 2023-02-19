use sqlx::{Pool, Sqlite};
use std::error::Error;

use rec::model::relations::{
    ShiftProblemProblem,
    ShiftProblemSparePart
};
use sqlx::query;

pub async fn save_problem(pool : &Pool<Sqlite>,
                sp : ShiftProblemProblem) -> Result<(),Box<dyn Error>> {
  let ShiftProblemProblem{problem_id,shift_problem_id} = sp;
  let problem_id = problem_id.to_string();
  let shift_problem_id = shift_problem_id.to_string();

  let row = query!("
    INSERT INTO shift_problem_problem(
        shift_problem_id,
        problem_id)
    VALUES($1,$2);",
    shift_problem_id,problem_id)
    .execute(pool);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn delete_problem(pool : &Pool<Sqlite>,
            sp : ShiftProblemProblem) -> Result<(),Box<dyn Error>> {
  let ShiftProblemProblem{problem_id,shift_problem_id} = sp;
  let problem_id = problem_id.to_string();
  let shift_problem_id = shift_problem_id.to_string();

  let row = query!("
    DELETE FROM shift_problem_problem
    WHERE shift_problem_id = $1 AND problem_id = $2;",
    shift_problem_id,problem_id)
    .execute(pool);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn save_spare_part(pool : &Pool<Sqlite>,
            ss : ShiftProblemSparePart) -> Result<(),Box<dyn Error>> {
  let ShiftProblemSparePart{shift_problem_id,spare_part_id} = ss;
  let spare_part_id = spare_part_id.to_string();
  let shift_problem_id = shift_problem_id.to_string();

  let row = query!("
    INSERT INTO shift_problem_spare_part(
        shift_problem_id,
        spare_part_id)
    VALUES($1,$2);",
    shift_problem_id,spare_part_id)
    .execute(pool);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn delete_spare_part(pool : &Pool<Sqlite>,
            ss : ShiftProblemSparePart) -> Result<(),Box<dyn Error>> {
  let ShiftProblemSparePart{shift_problem_id,spare_part_id} = ss;
  let spare_part_id = spare_part_id.to_string();
  let shift_problem_id = shift_problem_id.to_string();

  let row = query!("
    DELETE FROM shift_problem_spare_part
    WHERE shift_problem_id = $1 AND spare_part_id = $2;",
    shift_problem_id,spare_part_id)
    .execute(pool);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}
