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

#[derive(Serialize,Deserialize,Debug)]
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
  pub note              : Option<Note>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Note{
  pub id   : Option<Uuid>,
  pub content : String
}
