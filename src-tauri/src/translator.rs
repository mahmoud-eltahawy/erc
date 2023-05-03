use rec::model::{permissions::PermissionName, shift::ShiftOrder};

pub fn translate_date(date: String) -> Vec<String> {
    let date = validate_date(date);
    return date
        .split('-')
        .into_iter()
        .map(|num| {
            let elm = num.chars().map(|c| translate_num(c)).collect::<String>();
            if &elm[0..2] == "٠" {
                return elm[2..].to_string();
            } else {
                return elm;
            }
        })
        .rev()
        .collect();
}

fn validate_date(date: String) -> String {
    if date.len() == 10 {
        return date;
    } else if date.len() == 12 {
        return date[1..11].to_string();
    } else {
        return "date-translate-error".to_string();
    }
}

pub fn translate_order(order: &ShiftOrder) -> String {
    match order {
        ShiftOrder::FIRST => "الاولي".to_owned(),
        ShiftOrder::SECOND => "الثانية".to_owned(),
        ShiftOrder::THIRD => "الثالثة".to_owned(),
    }
}

pub fn translate_num(num: char) -> char {
    match num {
        '1' => '١',
        '2' => '٢',
        '3' => '٣',
        '4' => '٤',
        '5' => '٥',
        '6' => '٦',
        '7' => '٧',
        '8' => '٨',
        '9' => '٩',
        '0' => '٠',
        _ => '_',
    }
}

pub fn translate_permission(permission: &PermissionName) -> String {
    match permission {
        PermissionName::WriteDepartmentProblem => "ادخال عطل".to_string(),
        PermissionName::ReadDepartmentProblems => "قراءة الاعطال".to_string(),
        PermissionName::DefineProblem => "تعريف مشكلة".to_string(),
        PermissionName::ModifyDepartmentProblems => "تعديل الاعطال".to_string(),
        PermissionName::AccessHistoryAllDepartmentsDepartmentProblems => {
            "قراءة سجل اعطال جميع الاقسام".to_string()
        }
        PermissionName::AccessHistoryDepartmentDepartmentProblems => {
            "قراءة سجل الاعطال الخاصة بالقسم فقط".to_string()
        }
        PermissionName::AccessHistoryAllDepartmentsProblems => {
            "قراءة سجل مشاكل جميع الاقسام".to_string()
        }
        PermissionName::AccessHistoryDepartmentProblems => {
            "قراءة سجل المشاكل الخاصة بالقسم فقط".to_string()
        }
        PermissionName::AccessHistoryEmployees => "قراءة سجل الموظفين".to_string(),
        PermissionName::AccessHistoryMachines => "قراءة سجل الماكينات".to_string(),
        PermissionName::AccessHistorySpareParts => "قراءة سجل قطع الغيار".to_string(),
    }
}
