use std::error::Error;

use rec::model::{employee::Employee, spare_part::SparePart};
use sqlx::Row;
use uuid::Uuid;

use crate::{config::AppState, api::{employee::save_employee, spare_parts::save_spare_part}};

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

  let spare_parts = vec![
    SparePart{id : Uuid::new_v4(), name : "part 1".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 2".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 3".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 4".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 5".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 6".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 7".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 8".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 9".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 10".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 11".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 12".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 13".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 14".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 15".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 16".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 17".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 18".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 19".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 20".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 21".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 22".to_string()},
    SparePart{id : Uuid::new_v4(), name : "part 23".to_string()},
  ];


  let q = sqlx::query(r#"
      SELECT count(id) FROM spare_part;
  "#).fetch_one(&app_state.pool).await?;

  let count : i64 = q.get(0);

  if count < 20 {
    for s in spare_parts {
      save_spare_part(app_state, &s).await?;
    }
  }
  Ok(())
}
