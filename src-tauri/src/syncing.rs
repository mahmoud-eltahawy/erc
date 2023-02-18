use std::error::Error;

use rec::crud_sync::{CudVersion, Cud, Table};
use uuid::Uuid;

use crate::{
    memory::{
    syncing::{last_version, save_version},
    employee::{self, delete_employee_by_id},
    shift::{save_shift, delete_shift_by_id},
    problem::{self, delete_problem_by_id},
    spare_part::{save_spare_part, delete_spare_part_by_id, update_spare_part}
  },
  config::AppState,
  api::{
      syncing::fetch_updates,
      employee::fetch_employee_by_id,
      shift::fetch_shift_by_id,
      problem::fetch_problem_by_id, spare_parts::fetch_spare_part_by_id
  },
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
       Table::Problem               => {
           let problem = fetch_problem_by_id(app_state, target_id).await?;
           problem::save(&app_state.pool, problem).await?;
       },
       Table::Shift                 => {
           let shift = fetch_shift_by_id(app_state, &target_id).await?;
           save_shift(&app_state.pool,shift).await?
       },
       Table::SparePart             => {
           let part = fetch_spare_part_by_id(app_state, target_id).await?;
           save_spare_part(&app_state.pool, part).await?;
       },
       Table::Department            => {println!("unimplemented")},
       Table::Machine               => {println!("unimplemented")},
       Table::ShiftNote             => {println!("unimplemented")},
       Table::ShiftProblem          => {println!("unimplemented")},
       Table::ShiftProblemNote      => {println!("unimplemented")},
       Table::ShiftProblemProblem   => {println!("unimplemented")},
       Table::ShiftProblemSparePart => {println!("unimplemented")},
       Table::Undefined             => return Err("undefined table".into())
   }
   Ok(())
}

async fn delete(app_state : &AppState, target_id : Uuid,table : Table,_other_id : Option<Uuid>) -> Result<(),Box<dyn Error>>{
   match table {
       Table::Employee              => delete_employee_by_id(&app_state.pool, target_id.to_string()).await?,
       Table::Problem               => delete_problem_by_id(&app_state.pool, target_id.to_string()).await?,
       Table::Shift                 => delete_shift_by_id(&app_state.pool, target_id.to_string()).await?,
       Table::SparePart             => delete_spare_part_by_id(&app_state.pool, target_id.to_string()).await?,
       Table::Machine               => {println!("unimplemented")},
       Table::ShiftNote             => {println!("unimplemented")},
       Table::ShiftProblem          => {println!("unimplemented")},
       Table::ShiftProblemNote      => {println!("unimplemented")},
       Table::ShiftProblemProblem   => {println!("unimplemented")},
       Table::ShiftProblemSparePart => {println!("unimplemented")},
       Table::Department            => {println!("unimplemented")},
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
       Table::Problem  => {
           let problem = fetch_problem_by_id(app_state, target_id).await?;
           problem::update(&app_state.pool, problem).await?;
       },
       Table::SparePart             => {
           let part = fetch_spare_part_by_id(app_state, target_id).await?;
           update_spare_part(&app_state.pool, part).await?;
       },
       Table::Department            => {println!("unimplemented")},
       Table::Machine               => {println!("unimplemented")},
       Table::ShiftNote             => {println!("unimplemented")},
       Table::ShiftProblem          => {println!("unimplemented")},
       Table::ShiftProblemNote      => {println!("unimplemented")},
       Table::ShiftProblemProblem   => {println!("unimplemented")},
       Table::ShiftProblemSparePart => {println!("unimplemented")},
       Table::Shift                 => return Err("shift can not be updated".into()),
       Table::Undefined             => return Err("undefined table".into())
   }
   Ok(())
}
