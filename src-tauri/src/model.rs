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
