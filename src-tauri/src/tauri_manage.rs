mod models;
mod async_commands;
mod non_async_commands;
mod components_commands;
mod state;
mod puzzle_pieces;

use std::error::Error;
use tauri::{Wry, Builder};

pub async fn app() -> Result<Builder<Wry>,Box<dyn Error>>{
  match state::create_tauri_state().await {
    Ok(state) => Ok(puzzle_pieces::build_tauri(state)),
    Err(err) => Err(err)
  }
}
