use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::error::Error;

use crate::model::{Employee, Probelm, Machine, SparePart};

#[derive(Serialize,Deserialize,Clone)]
pub struct Name{
  pub id : Option<Uuid>,
  pub name : String
}

impl Name{
  fn build_employee(employee : &Employee) -> Name{
    Name {
      id: employee.id,
      name: format!("{} {} {}",
              employee.first_name,
              employee.middle_name,
              employee.last_name
      )
    }
  }

  fn build_problem(problem : &Probelm) -> Name{
    Name {
      id: Some(problem.id),
      name: problem.title.clone()
    }
  }

  fn build_machine(machine : &Machine) -> Name{
    Name {
      id: Some(machine.id),
      name: machine.name.clone()
    }
  }

  fn build_spare_part(spare_part : &SparePart) -> Name{
    Name {
      id: Some(spare_part.id),
      name: spare_part.name.clone()
    }
  }
}

pub async fn all_employees() -> Result<Vec<Name>,Box<dyn Error>> {
  let client = reqwest::Client::new();
  let result = client.get("http://127.0.0.1:8080/api/emp/all")
      .send()
      .await?
      .json::<Vec<Employee>>()
      .await?;

  let result : Vec<Name> = result
    .iter().map(|e| Name::build_employee(e)).collect();

  Ok(result)
}

pub async fn all_problems() -> Result<Vec<Name>,Box<dyn Error>> {
  let client = reqwest::Client::new();
  let result = client.get("http://127.0.0.1:8080/api/problem/all")
      .send()
      .await?
      .json::<Vec<Probelm>>()
      .await?;

  let result : Vec<Name> = result
    .iter().map(|p| Name::build_problem(p)).collect();

  Ok(result)
}

pub async fn all_machines() -> Result<Vec<Name>,Box<dyn Error>> {
  let client = reqwest::Client::new();
  let result = client.get("http://127.0.0.1:8080/api/machine/all")
      .send()
      .await?
      .json::<Vec<Machine>>()
      .await?;

  let result : Vec<Name> = result
    .iter().map(|p| Name::build_machine(p)).collect();

  Ok(result)
}

pub async fn all_spare_parts() -> Result<Vec<Name>,Box<dyn Error>> {
  let client = reqwest::Client::new();
  let result = client.get("http://127.0.0.1:8080/api/spare-part/all")
      .send()
      .await?
      .json::<Vec<SparePart>>()
      .await?;

  let result : Vec<Name> = result
    .iter().map(|s| Name::build_spare_part(s)).collect();

  Ok(result)
}
