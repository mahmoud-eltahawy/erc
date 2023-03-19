use rec::model::permissions::Permissions;
use sqlx::{Pool, Sqlite,Error, query};

pub async fn save(pool : &Pool<Sqlite>,permissions : Permissions) -> Result<(),Error> {
  let Permissions{
      id,
      define_problem,
      modify_department_problems,
      read_department_problems,
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      write_department_problem
  } = permissions;
  let row = query!("
    INSERT INTO permissions(id,
      define_problem,
      modify_department_problems,
      read_department_problems,
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      write_department_problem)
    VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12);",
      id,
      define_problem,
      modify_department_problems,
      read_department_problems,
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      write_department_problem).execute(pool);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn update(pool : &Pool<Sqlite>,permissions : Permissions) -> Result<(),Error> {
  let Permissions{
      id,
      define_problem,
      modify_department_problems,
      read_department_problems,
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      write_department_problem
  } = permissions;
  let row = query!("
    UPDATE permissions SET
      define_problem                                        = $2,
      modify_department_problems                            = $3,
      read_department_problems                              = $4,
      access_history_all_departments_department_problems    = $5,
      access_history_all_departments_problems               = $6,
      access_history_department_department_problems         = $7,
      access_history_department_problems                    = $8,
      access_history_employees                              = $9,
      access_history_machines                               = $10,
      access_history_spare_parts                            = $11,
      write_department_problem                              = $12
    WHERE id        = $1;",
      id,
      define_problem,
      modify_department_problems,
      read_department_problems,
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      write_department_problem).execute(pool);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}
