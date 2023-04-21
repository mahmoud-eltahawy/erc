use errc::translator::{translate_date, translate_order};
use rec::{
    model::employee::Employee,
    timer::{get_current_date, get_current_order, get_relative_now},
};
use std::sync::Mutex;
use tauri::Window;

#[tauri::command]
pub fn check_login(
    state: tauri::State<'_, Mutex<Option<(Employee<String>, String)>>>,
) -> (Option<Employee<String>>, Option<String>) {
    match &*state.lock().unwrap() {
        Some((employee, id)) => (Some(employee.clone()), Some(id.clone())),
        None => (None, None),
    }
}

#[tauri::command]
pub fn current_shift() -> Result<(String, Vec<String>), String> {
    let now = get_relative_now();
    let order = get_current_order(now);
    match get_current_date(now) {
        Some(date) => Ok((translate_order(&order), translate_date(date.to_string()))),
        None => Err("مشكلة داخلية في تحديث التاريخ".to_owned()),
    }
}

#[tauri::command]
pub fn logout(
    state: tauri::State<'_, Mutex<Option<(Employee<String>, String)>>>,
    window: Window,
) -> Result<(), String> {
    *state.lock().unwrap() = None;
    match window.emit("logout", None::<&str>) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
