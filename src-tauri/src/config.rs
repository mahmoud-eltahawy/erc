use std::env;

pub struct AppState{
  pub origin : String,
}

impl AppState{
  pub fn new() -> Self{
    let host = env::var("ERA_HOST").expect("invalid host key");
    let port = env::var("ERA_PORT").expect("invalid port key");

    AppState{
        origin : format!("http://{host}:{port}")
    }
  }
}
