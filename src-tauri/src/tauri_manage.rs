mod async_commands;
mod components_commands;
mod models;
mod non_async_commands;
mod puzzle_pieces;
mod state;

use std::error::Error;
use tauri::{Builder, Wry};

pub async fn app() -> Result<Builder<Wry>, Box<dyn Error>> {
    match state::create_tauri_state().await {
        Ok(state) => Ok(puzzle_pieces::build_tauri(state)),
        Err(err) => Err(err),
    }
}
