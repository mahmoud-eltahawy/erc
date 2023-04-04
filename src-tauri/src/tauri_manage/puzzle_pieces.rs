use std::sync::Mutex;
use rec::model::employee::Employee;
use tauri::{Builder, Wry};

use super::{
    async_commands::*,
    non_async_commands::*,
    components_commands::shift_problem_form::*,
    components_commands::history::*,
    components_commands::controll::*,
};
use super::models::TauriState;

pub fn build_tauri(state : TauriState) -> Builder<Wry>{
  let TauriState{app_state} = state;
  tauri::Builder::default()
    .manage(app_state)
    .manage(Mutex::new(None::<(Employee<String>,String)>))
    .invoke_handler(tauri::generate_handler![
      update,
      login,
      logout,
      check_login,
      current_shift,
      check_shift_time,
      current_shift_borders,
      get_current_shift_problems,
      define_problem,
      problems_selection,
      machines_selection,
      employees_selection,
      spare_parts_selection,
      save_problem_detail,
      get_employee_by_id,
      get_problem_by_id,
      get_spare_part_by_id,
      get_machine_by_id,
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
      employee_name,
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
    ])
}
