use std::error::Error;

use rec::crud_sync::{CudVersion, Cud, Table};
use uuid::Uuid;

use crate::{
    memory::{
    syncing::{last_version, save_version},
    employee
  },
  config::AppState,
  api::{
      syncing::fetch_updates,
      fetching::fetch_employee_by_id
  }
};

pub async fn upgrade(app_state : &AppState) -> Result<(),Box<dyn Error>> {
    let version = last_version(&app_state.pool).await?;
    let updates = fetch_updates(app_state, version as u64).await?;
    for update in updates {
        apply_update(app_state,update).await?
    }
    Ok(())
}

async fn apply_update(app_state : &AppState,cud_version : CudVersion) -> Result<(),Box<dyn Error>>{
    let CudVersion{version_number : _,cud,target_id,target_table,other_target_id} = cud_version;
    match cud {
        Cud::Create     => create(app_state,target_id, target_table, other_target_id).await?,
        Cud::Delete     => delete(app_state,target_id, target_table, other_target_id).await?,
        Cud::Update     => update(app_state,target_id, target_table, other_target_id).await?,
        Cud::Undefined  => return Err("undefined crud".into())
    }
    save_version(&app_state.pool, cud_version).await?;
    Ok(())
}

async fn create(app_state : &AppState, target_id : Uuid,table : Table,_other_id : Option<Uuid>) -> Result<(),Box<dyn Error>>{
   match table {
       Table::Employee              => {
          let employee = fetch_employee_by_id(app_state, target_id).await?;
          employee::save(&app_state.pool, employee).await?
       },
       Table::Department            => {println!("unimplemented")},
       Table::Machine               => {println!("unimplemented")},
       Table::Problem               => {println!("unimplemented")},
       Table::Shift                 => {println!("unimplemented")},
       Table::ShiftNote             => {println!("unimplemented")},
       Table::ShiftProblem          => {println!("unimplemented")},
       Table::ShiftProblemNote      => {println!("unimplemented")},
       Table::ShiftProblemProblem   => {println!("unimplemented")},
       Table::ShiftProblemSparePart => {println!("unimplemented")},
       Table::SparePart             => {println!("unimplemented")},
       Table::Undefined             => return Err("undefined table".into())
   }
   Ok(())
}

async fn delete(app_state : &AppState, target_id : Uuid,table : Table,_other_id : Option<Uuid>) -> Result<(),Box<dyn Error>>{
   match table {
       Table::Employee => {
          employee::delete_by_id(&app_state.pool, target_id.to_string()).await?
       },
       Table::Department            => {println!("unimplemented")},
       Table::Machine               => {println!("unimplemented")},
       Table::Problem               => {println!("unimplemented")},
       Table::Shift                 => {println!("unimplemented")},
       Table::ShiftNote             => {println!("unimplemented")},
       Table::ShiftProblem          => {println!("unimplemented")},
       Table::ShiftProblemNote      => {println!("unimplemented")},
       Table::ShiftProblemProblem   => {println!("unimplemented")},
       Table::ShiftProblemSparePart => {println!("unimplemented")},
       Table::SparePart             => {println!("unimplemented")},
       Table::Undefined             => return Err("undefined table".into())
   }
   Ok(())
}

async fn update(app_state : &AppState, target_id : Uuid,table : Table,_other_id : Option<Uuid>) -> Result<(),Box<dyn Error>>{
   match table {
       Table::Employee => {
          let employee = fetch_employee_by_id(app_state, target_id).await?;
          employee::update(&app_state.pool, employee).await?
       },
       Table::Department            => {println!("unimplemented")},
       Table::Machine               => {println!("unimplemented")},
       Table::Problem               => {println!("unimplemented")},
       Table::Shift                 => {println!("unimplemented")},
       Table::ShiftNote             => {println!("unimplemented")},
       Table::ShiftProblem          => {println!("unimplemented")},
       Table::ShiftProblemNote      => {println!("unimplemented")},
       Table::ShiftProblemProblem   => {println!("unimplemented")},
       Table::ShiftProblemSparePart => {println!("unimplemented")},
       Table::SparePart             => {println!("unimplemented")},
       Table::Undefined             => return Err("undefined table".into())
   }
   Ok(())
}
