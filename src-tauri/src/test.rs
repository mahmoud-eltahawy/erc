use std::{error::Error, str::FromStr};

use chrono::{Local, NaiveDateTime};
use rec::{
    crud_sync::Version,
    model::{
        department::{Department, UpdateDepartment},
        employee::Employee,
        machine::Machine,
        spare_part::SparePart,
        Environment, TableCrud, TableRequest,
    },
};
use uuid::Uuid;

use crate::{api, config::AppState};

async fn is_first_shoot(app_state: &AppState) -> Result<bool, Box<dyn Error>> {
    let origin = &app_state.origin;
    let result = reqwest::Client::new()
        .get(format!("{origin}/app/0/0"))
        .send()
        .await?
        .json::<Vec<Version>>()
        .await?;

    return Ok(result.is_empty());
}

pub async fn insert_basic_data(app_state: &AppState) -> Result<(), Box<dyn Error>> {
    if !is_first_shoot(app_state).await? {
        return Ok(());
    }

    let zero_id = Uuid::nil();

    let department = Department {
        id: zero_id,
        name: "الادارة".to_string(),
        boss_id: None,
    };

    let now = Local::now().timestamp_millis();
    let now = NaiveDateTime::from_timestamp_millis(now);
    let Some(time_stamp) = now else {
        return Err("unvalid time".to_string().into());
    };

    api::main_entry(
        app_state,
        TableRequest::Department(TableCrud::Create(Environment {
            updater_id: zero_id,
            target: department,
            time_stamp,
        })),
    )
    .await?;

    let employee = Employee {
        id: zero_id,
        card_id: 0,
        first_name: "e".to_string(),
        middle_name: "r".to_string(),
        last_name: "c".to_string(),
        position: "SUPER_USER".to_string(),
        department_id: zero_id,
        password: "1234".to_string(),
    };
    api::main_entry(
        app_state,
        TableRequest::Employee(TableCrud::Create(Environment {
            updater_id: zero_id,
            target: employee,
            time_stamp,
        })),
    )
    .await?;

    api::main_entry(
        app_state,
        TableRequest::Department(TableCrud::Update(Environment {
            updater_id: zero_id,
            target: UpdateDepartment::SetBoss(zero_id, zero_id),
            time_stamp,
        })),
    )
    .await?;

    insert_employees(app_state).await?; //TODO : remove this line in production

    Ok(())
}

async fn insert_employees(app_state: &AppState) -> Result<(), Box<dyn Error>> {
    let now = Local::now().timestamp_millis();
    let now = NaiveDateTime::from_timestamp_millis(now);
    let Some(time_stamp) = now else {
        return Err("unvalid time".to_string().into());
    };

    let zero_id = Uuid::from_str("00000000-0000-0000-0000-000000000000").expect("unvalid uuid");

    let kilens_id = Uuid::new_v4();
    let drayers_id = Uuid::new_v4();
    let incjet_id = Uuid::new_v4();

    let departments = vec![
        Department {
            id: kilens_id,
            name: "الافران".to_string(),
            boss_id: None,
        },
        Department {
            id: drayers_id,
            name: "المجففات".to_string(),
            boss_id: None,
        },
        Department {
            id: incjet_id,
            name: "الانكجت".to_string(),
            boss_id: None,
        },
    ];
    for target in departments {
        api::main_entry(
            app_state,
            TableRequest::Department(TableCrud::Create(Environment {
                updater_id: zero_id,
                target,
                time_stamp,
            })),
        )
        .await?;
    }

    let employee1 = Employee {
        id: Uuid::new_v4(),
        card_id: 1,
        department_id: kilens_id,
        first_name: "احمد".to_string(),
        middle_name: "جمال".to_string(),
        last_name: "محمد".to_string(),
        password: "1234".to_string(),
        position: "USER".to_string(),
    };

    let employee2 = Employee {
        department_id: drayers_id,
        ..employee1.clone()
    };
    let employee3 = Employee {
        department_id: incjet_id,
        ..employee1.clone()
    };
    let employee4 = Employee {
        department_id: kilens_id,
        ..employee1.clone()
    };

    let id1 = Uuid::new_v4();
    let id2 = Uuid::new_v4();
    let id3 = Uuid::new_v4();
    let id4 = Uuid::new_v4();
    let id5 = Uuid::new_v4();
    let id6 = Uuid::new_v4();
    let id7 = Uuid::new_v4();
    let id8 = Uuid::new_v4();
    let id9 = Uuid::new_v4();
    let id10 = Uuid::new_v4();
    let id11 = Uuid::new_v4();
    let id12 = Uuid::new_v4();
    let id13 = Uuid::new_v4();
    let id14 = Uuid::new_v4();
    let id15 = Uuid::new_v4();
    let id16 = Uuid::new_v4();
    let id17 = Uuid::new_v4();
    let id18 = Uuid::new_v4();
    let id19 = Uuid::new_v4();
    let id20 = Uuid::new_v4();
    let id21 = Uuid::new_v4();
    let id22 = Uuid::new_v4();
    let id23 = Uuid::new_v4();
    let id24 = Uuid::new_v4();
    let id25 = Uuid::new_v4();

    let employees: Vec<Employee> = vec![
        Employee {
            id: id1,
            card_id: 1,
            first_name: "احمد".to_string(),
            ..employee1.clone()
        },
        Employee {
            id: id2,
            card_id: 2,
            first_name: "علي".to_string(),
            ..employee1.clone()
        },
        Employee {
            id: id3,
            card_id: 3,
            first_name: "صابر".to_string(),
            ..employee1.clone()
        },
        Employee {
            id: id4,
            card_id: 4,
            first_name: "صلاح".to_string(),
            ..employee1.clone()
        },
        Employee {
            id: id5,
            card_id: 5,
            first_name: "جابر".to_string(),
            ..employee1.clone()
        },
        Employee {
            id: id6,
            card_id: 6,
            first_name: "جلال".to_string(),
            ..employee2.clone()
        },
        Employee {
            id: id7,
            card_id: 7,
            first_name: "يوسف".to_string(),
            ..employee2.clone()
        },
        Employee {
            id: id8,
            card_id: 8,
            first_name: "هلال".to_string(),
            ..employee2.clone()
        },
        Employee {
            id: id9,
            card_id: 9,
            first_name: "ابراهيم".to_string(),
            ..employee2.clone()
        },
        Employee {
            id: id10,
            card_id: 10,
            first_name: "منتصر".to_string(),
            ..employee2.clone()
        },
        Employee {
            id: id11,
            card_id: 11,
            first_name: "عمر".to_string(),
            ..employee3.clone()
        },
        Employee {
            id: id12,
            card_id: 12,
            first_name: "حكيم".to_string(),
            ..employee3.clone()
        },
        Employee {
            id: id13,
            card_id: 13,
            first_name: "طه".to_string(),
            ..employee3.clone()
        },
        Employee {
            id: id14,
            card_id: 14,
            first_name: "علاء".to_string(),
            ..employee3.clone()
        },
        Employee {
            id: id15,
            card_id: 15,
            first_name: "مصطفي".to_string(),
            ..employee3.clone()
        },
        Employee {
            id: id16,
            card_id: 16,
            first_name: "معتز".to_string(),
            ..employee4.clone()
        },
        Employee {
            id: id17,
            card_id: 17,
            first_name: "سيد".to_string(),
            ..employee4.clone()
        },
        Employee {
            id: id18,
            card_id: 18,
            first_name: "اسماعيل".to_string(),
            ..employee4.clone()
        },
        Employee {
            id: id19,
            card_id: 19,
            first_name: "جمعة".to_string(),
            ..employee4.clone()
        },
        Employee {
            id: id20,
            card_id: 20,
            first_name: "اشرف".to_string(),
            ..employee4.clone()
        },
        Employee {
            id: id21,
            card_id: 21,
            first_name: "طارق".to_string(),
            ..employee4.clone()
        },
        Employee {
            id: id22,
            card_id: 22,
            first_name: "يحي".to_string(),
            ..employee4.clone()
        },
        Employee {
            id: id23,
            card_id: 23,
            first_name: "مهدي".to_string(),
            ..employee4.clone()
        },
        Employee {
            id: id24,
            card_id: 24,
            first_name: "شعبان".to_string(),
            ..employee4.clone()
        },
        Employee {
            id: id25,
            card_id: 25,
            first_name: "عباس".to_string(),
            ..employee4.clone()
        },
    ];

    for target in employees {
        api::main_entry(
            app_state,
            TableRequest::Employee(TableCrud::Create(Environment {
                updater_id: zero_id,
                target,
                time_stamp,
            })),
        )
        .await?;
    }

    let spare_parts = vec![
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 1".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 2".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 3".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 4".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 5".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 6".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 7".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 8".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 9".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 10".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 11".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 12".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 13".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 14".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 15".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 16".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 17".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 18".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 19".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 20".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 21".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 22".to_string(),
        },
        SparePart {
            id: Uuid::new_v4(),
            name: "قطعة 23".to_string(),
        },
    ];

    for target in spare_parts {
        api::main_entry(
            app_state,
            TableRequest::SparePart(TableCrud::Create(Environment {
                updater_id: zero_id,
                target,
                time_stamp,
            })),
        )
        .await?;
    }

    let machines = vec![
        Machine {
            id: Uuid::new_v4(),
            name: "فرن 1".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "فرن 2".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "فرن 3".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "فرن 4".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "فرن 5".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "فرن 6".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "مجفف 1".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "مجفف 2".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "مجفف 3".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "مجفف 4".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "مجفف 5".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "مجفف 6".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "مجفف 7".to_string(),
        },
        Machine {
            id: Uuid::new_v4(),
            name: "مجفف 8".to_string(),
        },
    ];

    for target in machines {
        api::main_entry(
            app_state,
            TableRequest::Machine(TableCrud::Create(Environment {
                updater_id: zero_id,
                target,
                time_stamp,
            })),
        )
        .await?;
    }

    Ok(())
}
