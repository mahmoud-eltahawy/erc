use std::error::Error;

use rec::{model::{employee::Employee, spare_part::SparePart, department::Department, machine::Machine}, crud_sync::CudVersion};
use uuid::Uuid;

use crate::{
  config::AppState,
  api::{
    employee::save_employee,
    spare_parts::save_spare_part,
    department::save_department, machine::save_machine
  }
};

pub async fn insert_employees(app_state : &AppState) -> Result<(),Box<dyn Error>> {

  let origin = &app_state.origin;
  let result = reqwest::Client::new()
    .get(format!("{origin}/sync/1"))
    .send()
    .await?
    .json::<Vec<CudVersion>>()
    .await?;

  if !result.is_empty() {
    return Ok(());
  }

  let kilens_id   = Uuid::new_v4();
  let drayers_id  = Uuid::new_v4();
  let incjet_id   = Uuid::new_v4();
  let watch_id    = Uuid::new_v4();
  let sort_id     = Uuid::new_v4();

  let departments = vec![
    Department{id : kilens_id ,name: "الافران".to_string()  ,boss_id: None,department_id:None},
    Department{id : drayers_id,name: "المجففات".to_string() ,boss_id: None,department_id:None},
    Department{id : incjet_id ,name: "الانكجت".to_string(),boss_id: None,department_id:None},
    Department{id : watch_id  ,name: "المتابعة".to_string()   ,boss_id: None,department_id:None},
    Department{id : sort_id   ,name: "الفرز".to_string()    ,boss_id: None,department_id:None},
  ];
  for d in departments {
      save_department(app_state, &d).await?;
  }

  let employees : Vec<Employee> = vec![
    Employee{id : Uuid::new_v4(),card_id : 1,department_id  : kilens_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 2,department_id  : kilens_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 3,department_id  : kilens_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 4,department_id  : kilens_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 5,department_id  : drayers_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 6,department_id  : drayers_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 7,department_id  : drayers_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 8,department_id  : drayers_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 9,department_id  : incjet_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 10,department_id : incjet_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 11,department_id : incjet_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 12,department_id : incjet_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 13,department_id : watch_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 14,department_id : watch_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 15,department_id : watch_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 16,department_id : watch_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 17,department_id : sort_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 18,department_id : sort_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 19,department_id : sort_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
    Employee{id : Uuid::new_v4(),card_id : 20,department_id : sort_id,
        first_name : "احمد".to_string(),middle_name: "جمال".to_string(),last_name : "محمد".to_string(),password : "1234".to_string(),position : "ADMIN".to_string()},
  ];
  for e in employees {
      save_employee(app_state, &e).await?;
  }

  let spare_parts = vec![
    SparePart{id : Uuid::new_v4(), name : "قطعة 1".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 2".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 3".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 4".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 5".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 6".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 7".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 8".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 9".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 10".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 11".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 12".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 13".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 14".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 15".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 16".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 17".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 18".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 19".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 20".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 21".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 22".to_string()},
    SparePart{id : Uuid::new_v4(), name : "قطعة 23".to_string()},
  ];

  for s in spare_parts {
    save_spare_part(app_state, &s).await?;
  }

  let machines = vec![
    Machine{id: Uuid::new_v4(),name : "فرن 1".to_string()},
    Machine{id: Uuid::new_v4(),name : "فرن 2".to_string()},
    Machine{id: Uuid::new_v4(),name : "فرن 3".to_string()},
    Machine{id: Uuid::new_v4(),name : "فرن 4".to_string()},
    Machine{id: Uuid::new_v4(),name : "فرن 5".to_string()},
    Machine{id: Uuid::new_v4(),name : "فرن 6".to_string()},
    Machine{id: Uuid::new_v4(),name : "مجفف 1".to_string()},
    Machine{id: Uuid::new_v4(),name : "مجفف 2".to_string()},
    Machine{id: Uuid::new_v4(),name : "مجفف 3".to_string()},
    Machine{id: Uuid::new_v4(),name : "مجفف 4".to_string()},
    Machine{id: Uuid::new_v4(),name : "مجفف 5".to_string()},
    Machine{id: Uuid::new_v4(),name : "مجفف 6".to_string()},
    Machine{id: Uuid::new_v4(),name : "مجفف 7".to_string()},
    Machine{id: Uuid::new_v4(),name : "مجفف 8".to_string()},
  ];

  for m in machines {
    save_machine(app_state, &m).await?;
  }

  Ok(())
}
