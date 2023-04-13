use std::{error::Error, str::FromStr};

use rec::{
    crud_sync::CudVersion,
    model::{
        department::Department, employee::Employee, machine::Machine, permissions::Permissions,
        spare_part::SparePart,
    },
};
use uuid::Uuid;

use crate::{
    api::{
        department::{save_department, sets_department_boss},
        employee::save_employee,
        machine::save_machine,
        permissions::save_permissions,
        spare_parts::save_spare_part,
    },
    config::AppState,
};

async fn is_first_shoot(app_state: &AppState) -> Result<bool, Box<dyn Error>> {
    let origin = &app_state.origin;
    let result = reqwest::Client::new()
        .get(format!("{origin}/sync/1"))
        .send()
        .await?
        .json::<Vec<CudVersion>>()
        .await?;

    return Ok(result.is_empty());
}

pub async fn insert_basic_data(app_state: &AppState) -> Result<(), Box<dyn Error>> {
    if !is_first_shoot(app_state).await? {
        return Ok(());
    }

    let zero_id = Uuid::from_str("00000000-0000-0000-0000-000000000000").expect("unvalid uuid");

    let department = Department {
        id: zero_id,
        name: "الادارة".to_string(),
        boss_id: None,
    };

    save_department(app_state, &department).await?;

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

    save_employee(app_state, &employee).await?;

    let permissions = Permissions::default(zero_id);

    save_permissions(app_state, &permissions).await?;

    sets_department_boss(app_state, &zero_id).await?;

    insert_employees(app_state).await?; //TODO : remove this line in production

    Ok(())
}

async fn insert_employees(app_state: &AppState) -> Result<(), Box<dyn Error>> {
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
    for d in departments {
        save_department(app_state, &d).await?;
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

    let employees: Vec<Employee<Uuid>> = vec![
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

    for e in employees {
        save_employee(app_state, &e).await?;
    }

    let permissions = vec![
        Permissions::default(id1),
        Permissions::default(id2),
        Permissions::default(id3),
        Permissions::default(id4),
        Permissions::default(id5),
        Permissions::default(id6),
        Permissions::default(id7),
        Permissions::default(id8),
        Permissions::default(id9),
        Permissions::default(id10),
        Permissions::default(id11),
        Permissions::default(id12),
        Permissions::default(id13),
        Permissions::default(id14),
        Permissions::default(id15),
        Permissions::default(id16),
        Permissions::default(id17),
        Permissions::default(id18),
        Permissions::default(id19),
        Permissions::default(id20),
        Permissions::default(id21),
        Permissions::default(id22),
        Permissions::default(id23),
        Permissions::default(id24),
        Permissions::default(id25),
    ];

    for p in permissions {
        save_permissions(app_state, &p).await?;
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

    for s in spare_parts {
        save_spare_part(app_state, &s).await?;
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

    for m in machines {
        save_machine(app_state, &m).await?;
    }

    Ok(())
}
