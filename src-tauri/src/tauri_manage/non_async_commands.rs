use std::sync::Mutex;
use rec::{
  timer::{
    get_current_date,
    get_relative_now, get_current_order
  },
  model::employee::ClientEmployee
};
use errc::translator::{
  translate_date,
  translate_order
};
use uuid::Uuid;

#[tauri::command]
pub fn check_login(state : tauri::State<'_,Mutex<Option<(ClientEmployee,Uuid)>>>) -> Result<(ClientEmployee,Uuid),String> {
  match &*state.lock().unwrap() {
    Some((employee,id)) => Ok((employee.clone(),id.clone())),
    None     => Err("تحتاج الي تسجيل الدخول من جديد".to_string())
  }
}

#[tauri::command]
pub fn current_shift() -> Result<(String,Vec<String>),String> {
  let now = get_relative_now();
  let order = get_current_order(now);
  match get_current_date(now) {
    Some(date) => Ok((translate_order(&order),translate_date(date.to_string()))),
    None       => Err("مشكلة داخلية في تحديث التاريخ".to_owned())
  }
}

#[tauri::command]
pub fn logout(state : tauri::State<'_,Mutex<Option<(ClientEmployee,Uuid)>>>) {
  *state.lock().unwrap() = None;
}
