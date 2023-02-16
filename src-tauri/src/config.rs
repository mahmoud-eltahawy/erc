use std::env;

use sqlx::{Sqlite, Pool};

pub struct AppState{
  pub origin : String,
  pub pool   : Pool<Sqlite>
}

impl AppState{
  pub fn new(pool : Pool<Sqlite>) -> Self{
    let host = env::var("ERA_HOST").expect("invalid host key");
    let port = env::var("ERA_PORT").expect("invalid port key");

    AppState{
        origin : format!("http://{host}:{port}"),
        pool
    }
  }
}
