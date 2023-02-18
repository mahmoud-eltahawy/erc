use std::{sync::Mutex, collections::HashMap};
use rec::{
    timer::get_shift_borders,
    model::{employee::ClientEmployee, shift_problem::MinimamlShiftProblem}
};
use tauri::{Builder, Wry};
use uuid::Uuid;

use super::{
    async_commands::*,
    non_async_commands::*
};
use super::models::{Employees, Machines, SpareParts, TauriState};

pub fn build_tauri(state : TauriState) -> Builder<Wry>{
  let TauriState{app_state,employees,machines,order,relative_now,spare_parts} = state;
  tauri::Builder::default()
    .manage(app_state)
    .manage(Mutex::new(relative_now))
    .manage(Mutex::new(order.clone()))
    .manage(Mutex::new(get_shift_borders(order)))
    .manage(Mutex::new(None::<(ClientEmployee,Uuid)>))
    .manage(Mutex::new(None::<(String,Vec<String>)>))
    .manage(Mutex::new(HashMap::<Uuid,Vec<MinimamlShiftProblem>>::new()))
    .manage(Employees(Mutex::new(employees)))
    .manage(Machines(Mutex::new(machines)))
    .manage(SpareParts(Mutex::new(spare_parts)))
    .invoke_handler(tauri::generate_handler![
      login,
      logout,
      check_login,
      current_shift,
      current_shift_borders,
      update_current_shift,
      update_current_shift_problems,
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
    ])
}
