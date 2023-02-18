use std::error::Error;

use rec::model::employee::Employee;
use sqlx::Row;
use uuid::Uuid;

use crate::{config::AppState, api::employee::save_employee};

pub async fn insert_employees(app_state : &AppState) -> Result<(),Box<dyn Error>> {
  let employees : Vec<Employee> = vec![
    Employee{id : Uuid::new_v4(),card_id : 1,department_id  : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 2,department_id  : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "mohammed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 3,department_id  : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "mohammed".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 4,department_id  : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "alaa".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 5,department_id  : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 6,department_id  : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 7,department_id  : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 8,department_id  : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 9,department_id  : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 10,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 11,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 12,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 13,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 14,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 15,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 16,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 17,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 18,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 19,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 20,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 21,department_id : Uuid::parse_str("ffade865-925f-43ee-8379-884dd05ca5eb")?,first_name : "ahmed".to_string(),middle_name: "gamal".to_string(),last_name : "mohammed".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()}
  ];

  let q = sqlx::query(r#"
      SELECT count(id) FROM employee;
  "#).fetch_one(&app_state.pool).await?;

  let count : i64 = q.get(0);

  if count < 20 {
    for e in employees {
        save_employee(app_state, &e).await?;
    }
  }
  Ok(())
}
