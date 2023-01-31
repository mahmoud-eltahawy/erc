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

#[derive(Serialize,Deserialize)]
pub struct Probelm{
  pub id          : Uuid,
  pub title       : String,
  pub description : String
}

#[derive(Serialize,Deserialize)]
pub struct Machine{
  pub id          : Uuid,
  pub name        : String
}

#[derive(Serialize,Deserialize)]
pub struct SparePart{
  pub id         : Uuid,
  pub name       : String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ProblemDetail{
  id                : Option<Uuid>,
  shift_id          : Uuid,
  writer_id         : Uuid,
  maintainer_id     : Uuid,
  machine_id        : Uuid,
  begin_time        : NaiveTime,
  end_time          : NaiveTime,
  problems_ids      : Vec<Uuid>,
  spare_parts_ids   : Option<Vec<Uuid>>,
  note              : Option<String>
}
