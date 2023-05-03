use rec::model::employee::Employee;
use std::sync::Mutex;
use tauri::{Builder, Wry};
use uuid::Uuid;

use super::models::TauriState;
use super::{
    async_commands::*, components_commands::controll::*, components_commands::history::*,
    components_commands::shift_problem_form::*, non_async_commands::*,
};

pub fn build_tauri(state: TauriState) -> Builder<Wry> {
    let TauriState { app_state } = state;
    tauri::Builder::default()
        .manage(app_state)
        .manage(Mutex::new(None::<(Employee, Uuid)>))
        //is the update func invoked
        .manage(Mutex::new(false))
        .invoke_handler(tauri::generate_handler![
            update,
            login,
            logout,
            check_login,
            current_shift,
            check_shift_time,
            current_shift_borders,
            define_problem,
            problems_selection,
            machines_selection,
            employees_selection,
            spare_parts_selection,
            save_problem_detail,
            update_problem_detail,
            get_shift_problem_by_id,
            get_shift_problems_ids_by_shift_id,
            get_shift_problem_spare_parts_ids_by_id,
            get_shift_problem_problems_ids_by_id,
            get_shift_problem_note_by_id,
            get_employee_by_id,
            employee_name,
            get_problem_name_by_id,
            get_spare_part_name_by_id,
            get_machine_name_by_id,
            search_shifts,
            search_problem,
            profile_problem,
            search_parts,
            search_machines,
            search_employees,
            search_department_employees,
            search_admins,
            search_non_admins,
            admin_employee,
            unadmin_employee,
            list_departments,
            find_department,
            department_employees,
            boss_employee,
            department_permissions,
            employee_permissions,
            employee_permissions_classified,
            permission_allow,
            permission_forbid,
            remove_shift_employee,
            add_shift_employee,
            save_shift_note,
            fetch_shift_notes_ids,
            fetch_shift_note,
            remove_shift_note,
            upgrade_shift_note,
            shift_existing_employees,
            shift_non_existing_employees,
            remove_shift_problem,
        ])
}
