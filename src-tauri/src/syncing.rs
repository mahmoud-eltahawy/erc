mod memory;
mod api;

use crate::config::AppState;
use std::error::Error;

use rec::{
    crud_sync::{
        CudVersion,
        Cud, Table
    },
    model::note::Note,
};
use tauri::Window;
use uuid::Uuid;

use memory::{
    syncing,
    shift,
    employee,
    problem,
    spare_part,
    department,
    machine,
    note,
    shift_problem,
    relations
};

use self::memory::permissions;

pub async fn upgrade(app_state : &AppState,window :Option<&Window>) -> Result<(),Box<dyn Error>> {
    let version = syncing::last_version(&app_state.pool).await?;
    let updates = api::updates(app_state, version as u64).await?;
    for update in updates {
        apply_update(app_state,update,window).await?
    }
    Ok(())
}

async fn apply_update(app_state : &AppState,cud_version : CudVersion,window : Option<&Window>) -> Result<(),Box<dyn Error>>{
  let CudVersion{version_number : _,cud,target_id,target_table,other_target_id} = cud_version;
  match target_table {
    Table::Employee              => update_employee(app_state, cud, target_id,window).await?,
    Table::Problem               => update_problem(app_state, cud, target_id,window).await?,
    Table::SparePart             => update_spare_part(app_state, cud, target_id,window).await?,
    Table::Machine               => update_machine(app_state, cud, target_id,window).await?,
    Table::ShiftProblem          => update_shift_problem(app_state, cud, target_id,window).await?,
    Table::Shift                 => update_shift(app_state, cud, target_id).await?,
    Table::Department            => update_department(app_state, cud, target_id).await?,
    Table::ShiftNote             => update_shift_note(app_state, cud, target_id).await?,
    Table::ShiftProblemNote      => update_shift_problem_note(app_state, cud, target_id).await?,
    Table::DepartmentShift       => update_department_shift(app_state, cud, target_id).await?,
    Table::ShiftProblemProblem   => update_shift_problem_problem(app_state, cud, target_id, other_target_id).await?,
    Table::ShiftProblemSparePart => update_shift_problem_spare_part(app_state, cud, target_id, other_target_id).await?,
    Table::Permissions           => update_permissions(app_state, cud, target_id).await?,
    Table::Undefined             => return Err("undefined table".into())
  }
  syncing::save_version(&app_state.pool, cud_version).await?;
  Ok(())
}

async fn update_permissions(app_state : &AppState,cud : Cud,target_id : Uuid) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      let per = api::permissions(app_state, target_id).await?;
      permissions::save(&app_state.pool, per).await?;
    }
    Cud::Update     => {
      let per = api::permissions(app_state, target_id).await?;
      permissions::update(&app_state.pool, per).await?;
    }
    Cud::Delete     => return Err("permissions can not be deleted".into()),
    Cud::Undefined  => return Err("undefined department crud".into())
  }

  Ok(())
}

async fn update_shift_problem(app_state : &AppState,cud : Cud,target_id : Uuid,window : Option<&Window>) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      let sp = api::shift_problem(app_state, target_id).await?;
      shift_problem::save(&app_state.pool, sp).await?;
    }
    Cud::Delete     => shift_problem::delete(&app_state.pool, target_id).await?,
    Cud::Update     => {
      let sp = api::shift_problem(app_state, target_id).await?;
      shift_problem::update(&app_state.pool, sp).await?;
    }
    Cud::Undefined  => return Err("undefined department crud".into())
  }

  if let Some(window) = window {
    window.emit("update_shift_problem","hello")?;
  }

  Ok(())
}

async fn update_employee(app_state : &AppState,cud : Cud,target_id : Uuid,window : Option<&Window>) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      let employee = api::employee(app_state, target_id).await?;
      employee::save(&app_state.pool, employee).await?;
    }
    Cud::Delete     => employee::delete(&app_state.pool, target_id).await?,
    Cud::Update     => {
      let employee = api::employee(app_state, target_id).await?;
      employee::update(&app_state.pool, employee).await?
    }
    Cud::Undefined  => return Err("undefined employee crud".into())
  }
  if let Some(window) = window {
    window.emit("update_employee","hello")?;
  }
  Ok(())
}

async fn update_problem(app_state : &AppState,cud : Cud,target_id : Uuid,window : Option<&Window>) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      let problem = api::problem(app_state, target_id).await?;
      problem::save(&app_state.pool, problem).await?;
    }
    Cud::Delete     => problem::delete(&app_state.pool, target_id).await?,
    Cud::Update     => {
      let problem = api::problem(app_state, target_id).await?;
      problem::update(&app_state.pool, problem).await?;
    }
    Cud::Undefined  => return Err("undefined employee crud".into())
  }
  if let Some(window) = window {
    window.emit("update_problem","hello")?;
  }
  Ok(())
}

async fn update_spare_part(app_state : &AppState,cud : Cud,target_id : Uuid,window :Option<&Window>) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      let part = api::spare_part(app_state, target_id).await?;
      spare_part::save(&app_state.pool, part).await?;
    }
    Cud::Delete     => spare_part::delete(&app_state.pool, target_id).await?,
    Cud::Update     => {
      let part = api::spare_part(app_state, target_id).await?;
      spare_part::update(&app_state.pool, part).await?;
    }
    Cud::Undefined  => return Err("undefined employee crud".into())
  }
  if let Some(window) = window {
    window.emit("update_spare_part","hello")?;
  }
  Ok(())
}

async fn update_machine(app_state : &AppState,cud : Cud,target_id : Uuid,window : Option<&Window>) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      let mac = api::machine(app_state, target_id).await?;
      machine::save(&app_state.pool, mac).await?;
    }
    Cud::Delete     => machine::delete(&app_state.pool, target_id).await?,
    Cud::Update     => {
      let mac = api::machine(app_state, target_id).await?;
      machine::update(&app_state.pool, mac).await?;
    }
    Cud::Undefined  => return Err("undefined department crud".into())
  }
  if let Some(window) = window {
    window.emit("update_machine","hello")?;
  }
  Ok(())
}

async fn update_department(app_state : &AppState,cud : Cud,target_id : Uuid) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      let dep = api::department(app_state, target_id).await?;
      department::save(&app_state.pool, dep).await?;
    }
    Cud::Delete     => department::delete(&app_state.pool, target_id).await?,
    Cud::Update     => {
      let dep = api::department(app_state, target_id).await?;
      department::update(&app_state.pool, dep).await?;
    }
    Cud::Undefined  => return Err("undefined department crud".into())
  }
  Ok(())
}

async fn update_shift_note(app_state : &AppState,cud : Cud,target_id : Uuid) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      let note = api::note(app_state, target_id).await?;
      note::save_to_shift(&app_state.pool, note).await?;
    }
    Cud::Delete     => note::delete(&app_state.pool, target_id).await?,
    Cud::Update     => {
      let note = api::note(app_state, target_id).await?;
      note::update(&app_state.pool, Note{id : note.id, content : note.content}).await?;
    }
    Cud::Undefined  => return Err("undefined department crud".into())
  }
  Ok(())
}

async fn update_shift_problem_note(app_state : &AppState,cud : Cud,target_id : Uuid) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      let note = api::note(app_state, target_id).await?;
      note::save_to_shift_problem(&app_state.pool, note).await?;
    }
    _               => return Err("note crud implemented in note section".into())
  }
  Ok(())
}

async fn update_shift(app_state : &AppState,cud : Cud,target_id : Uuid) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      let shift = api::shift(app_state, target_id).await?;
      shift::save(&app_state.pool,shift).await?
    }
    _               => return Err("shift is only created table".into()),
  }
  Ok(())
}

async fn update_department_shift(app_state : &AppState,cud : Cud,target_id : Uuid) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      let ds = api::shift_department(app_state, target_id).await?;
      shift::save_department_shift(&app_state.pool, ds).await?;
    },
    Cud::Delete     => shift::delete_department_shift(&app_state.pool, target_id).await?,
    _               => return Err("shift is only created or deleted table".into()),
  }
  Ok(())
}

async fn update_shift_problem_problem(app_state : &AppState,cud : Cud,target_id : Uuid,other_id : Option<Uuid>) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      match other_id {
        Some(id) => {
          relations::shift_problems::save_problem(&app_state.pool,target_id,id).await?;
        },
        None => return Err("the shift problem id is null".into())
      }
    },
    Cud::Delete     => {
      match other_id {
        Some(id) => {
          relations::shift_problems::delete_problem(&app_state.pool,target_id , id).await?
        },
        None => return Err("the shift problem id is null".into())
      }
    },
    _               => return Err("shift is only created or deleted table".into()),
  }
  Ok(())
}

async fn update_shift_problem_spare_part(app_state : &AppState,cud : Cud,target_id : Uuid,other_id : Option<Uuid>) -> Result<(),Box<dyn Error>>{
  match cud {
    Cud::Create     => {
      match other_id {
        Some(id) => {
          relations::shift_problems::save_spare_part(&app_state.pool,target_id,id).await?;
        },
        None => return Err("the shift problem id is null".into())
      }
    },
    Cud::Delete     =>{
      match other_id {
        Some(id) => {
          relations::shift_problems::delete_spare_part(&app_state.pool,target_id,id).await?
        },
        None => return Err("the shift problem id is null".into())
      }
    },
    _           => return Err("shift is only created or deleted table".into()),
  }
  Ok(())
}
