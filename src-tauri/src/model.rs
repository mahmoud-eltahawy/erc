use chrono::NaiveTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Employee{
  pub id            : Option<Uuid>,
  pub department_id : Uuid,
  pub position      : String,
  pub first_name    : String,
  pub middle_name   : String,
  pub last_name     : String,
  pub card_id       : i16,
  pub password      : String
}

#[derive(Serialize,Deserialize)]
pub struct Cred{
  pub card_id : i16,
  pub password: String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Probelm{
  pub id          : Uuid,
  pub title       : String,
  pub description : String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Machine{
  pub id          : Uuid,
  pub name        : String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct SparePart{
  pub id         : Uuid,
  pub name       : String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ShiftProblem{
  pub id                : Option<Uuid>,
  pub shift_id          : Uuid,
  pub writer_id         : Uuid,
  pub maintainer_id     : Uuid,
  pub machine_id        : Uuid,
  pub begin_time        : NaiveTime,
  pub end_time          : NaiveTime,
  pub problems_ids      : Vec<Uuid>,
  pub spare_parts_ids   : Option<Vec<Uuid>>,
  pub note              : Option<Note>
}

impl ShiftProblem {
  pub fn new(problem : ProblemDetail) -> Self{
    let ProblemDetail{
      id,
      shift_id,
      writer_id,
      begin_time,
      end_time,
      machine_id,
      problems_ids,
      spare_parts_ids,
      maintainer_id,
      note
    } = problem;
    let note = match note {
      Some(content) =>Some(Note{
        id : Uuid::new_v4(),
        content
      }),
      None => None
    };
    ShiftProblem {
      id,
      shift_id,
      writer_id,
      maintainer_id,
      machine_id,
      begin_time,
      end_time,
      problems_ids,
      spare_parts_ids,
      note
    }
  }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ProblemDetail{
  pub id                : Option<Uuid>,
  pub shift_id          : Uuid,
  pub writer_id         : Uuid,
  pub maintainer_id     : Uuid,
  pub machine_id        : Uuid,
  pub begin_time        : NaiveTime,
  pub end_time          : NaiveTime,
  pub problems_ids      : Vec<Uuid>,
  pub spare_parts_ids   : Option<Vec<Uuid>>,
  pub note              : Option<String>
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Note{
  pub id   : Uuid,
  pub content : String
}


#[derive(Serialize,Deserialize)]
pub struct Ids{
  pub writer_id     : Uuid,
  pub shift_id      : Uuid,
  pub department_id : Uuid
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Name{
  pub id : Option<Uuid>,
  pub name : String
}

impl Name{
  pub fn build_employee(employee : &Employee) -> Name{
    Name {
      id: employee.id,
      name: format!("{} {} {}",
              employee.first_name,
              employee.middle_name,
              employee.last_name
      )
    }
  }

  pub fn build_problem(problem : &Probelm) -> Name{
    Name {
      id: Some(problem.id),
      name: problem.title.clone()
    }
  }

  pub fn build_machine(machine : &Machine) -> Name{
    Name {
      id: Some(machine.id),
      name: machine.name.clone()
    }
  }

  pub fn build_spare_part(spare_part : &SparePart) -> Name{
    Name {
      id: Some(spare_part.id),
      name: spare_part.name.clone()
    }
  }
}
