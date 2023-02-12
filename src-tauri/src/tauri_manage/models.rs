use std::sync::Mutex;
use errc::config::AppState;
use rec::{model::name::Name, timer::ShiftOrder};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub struct Employees(pub Mutex<Vec<Name>>);
pub struct Machines(pub Mutex<Vec<Name>>);
pub struct SpareParts(pub Mutex<Vec<Name>>);


#[derive(Serialize,Deserialize)]
pub struct Ids{
  pub writer_id     : Uuid,
  pub shift_id      : Uuid,
  pub department_id : Uuid
}


pub struct TauriState {
 pub app_state     : AppState,
 pub relative_now  : i64,
 pub order         : ShiftOrder,
 pub employees     : Vec<Name>,
 pub machines      : Vec<Name>,
 pub spare_parts   : Vec<Name>,
}
