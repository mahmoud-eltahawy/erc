use std::{sync::Mutex, collections::HashMap};
use rec::model::shift_problem::ClientMinimamlShiftProblem;
use rec::model::employee::ClientEmployee;
use tauri::{Builder, Wry};
use uuid::Uuid;

use super::{
    async_commands::*,
    non_async_commands::*,
    components_commands::shift_problem_form::*,
};
use super::models::TauriState;

pub fn build_tauri(state : TauriState) -> Builder<Wry>{
  let TauriState{app_state} = state;
  tauri::Builder::default()
    .manage(app_state)
    .manage(Mutex::new(None::<(ClientEmployee,Uuid)>))
    .manage(Mutex::new(None::<(String,Vec<String>)>))
    .manage(Mutex::new(HashMap::<Uuid,Vec<ClientMinimamlShiftProblem>>::new()))
    .invoke_handler(tauri::generate_handler![
      login,
      logout,
      check_login,
      current_shift,
      current_shift_borders,
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
