use erc_ui::config::AppState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Ids {
    pub writer_id: Uuid,
    pub shift_id: Uuid,
    pub department_id: Uuid,
}

pub struct TauriState {
    pub app_state: AppState,
}
