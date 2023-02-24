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


pub async fn upgrade(app_state : &AppState) -> Result<(),Box<dyn Error>> {
    let version = syncing::last_version(&app_state.pool).await?;
    let updates = api::updates(app_state, version as u64).await?;
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
    syncing::save_version(&app_state.pool, cud_version).await?;
    Ok(())
}

async fn create(app_state : &AppState, target_id : Uuid,table : Table,other_id : Option<Uuid>) -> Result<(),Box<dyn Error>>{
   match table {
       Table::Employee              => {
           let employee = api::employee(app_state, target_id).await?;
           employee::save(&app_state.pool, employee).await?
       },
       Table::Problem               => {
           let problem = api::problem(app_state, target_id).await?;
           problem::save(&app_state.pool, problem).await?;
       },
       Table::Shift                 => {
           let shift = api::shift(app_state, target_id).await?;
           shift::save(&app_state.pool,shift).await?
       },
       Table::SparePart             => {
           let part = api::spare_part(app_state, target_id).await?;
           spare_part::save(&app_state.pool, part).await?;
       },
       Table::Department            => {
           let dep = api::department(app_state, target_id).await?;
           department::save(&app_state.pool, dep).await?;
       },
       Table::Machine               => {
           let mac = api::machine(app_state, target_id).await?;
           machine::save(&app_state.pool, mac).await?;
       },
       Table::ShiftNote             => {
           let note = api::note(app_state, target_id).await?;
           note::save_to_shift(&app_state.pool, note).await?;
       },
       Table::ShiftProblemNote      => {
           let note = api::note(app_state, target_id).await?;
           note::save_to_shift_problem(&app_state.pool, note).await?;
       },
       Table::ShiftProblem          => {
           let sp = api::shift_problem(app_state, target_id).await?;
           shift_problem::save(&app_state.pool, sp).await?;
       },
       Table::DepartmentShift       => {
           let ds = api::shift_department(app_state, target_id).await?;
           shift::save_department_shift(&app_state.pool, ds).await?;
       },
       Table::ShiftProblemProblem   => {
           match other_id {
               Some(id) => {
                  relations::shift_problems::save_problem(&app_state.pool,target_id,id).await?;
               },
               None => return Err("the shift problem id is null".into())
           }
       },
       Table::ShiftProblemSparePart => {
           match other_id {
               Some(id) => {
                  relations::shift_problems::save_spare_part(&app_state.pool,target_id,id).await?;
               },
               None => return Err("the shift problem id is null".into())
           }
       },
       Table::Undefined             => return Err("undefined table".into())
   }
   Ok(())
}

async fn delete(app_state : &AppState, target_id : Uuid,table : Table,other_id : Option<Uuid>) -> Result<(),Box<dyn Error>>{
   match table {
       Table::Employee              => employee::delete(&app_state.pool, target_id).await?,
       Table::Problem               => problem::delete(&app_state.pool, target_id).await?,
       Table::SparePart             => spare_part::delete(&app_state.pool, target_id).await?,
       Table::Department            => department::delete(&app_state.pool, target_id).await?,
       Table::Machine               => machine::delete(&app_state.pool, target_id).await?,
       Table::ShiftNote             => note::delete(&app_state.pool, target_id).await?,
       Table::ShiftProblem          => shift_problem::delete(&app_state.pool, target_id).await?,
       Table::DepartmentShift       => shift::delete_department_shift(&app_state.pool, target_id).await?,
       Table::ShiftProblemProblem   => {
           match other_id {
               Some(id) => {
                   relations::shift_problems::delete_problem(&app_state.pool,target_id , id).await?
               },
               None => return Err("the shift problem id is null".into())
           }
       },
       Table::ShiftProblemSparePart => {
           match other_id {
               Some(id) => {
                    relations::shift_problems::delete_spare_part(&app_state.pool,target_id,id).await?
               },
               None => return Err("the shift problem id is null".into())
           }
       },
       Table::Shift                 => return Err("shift is undeletable table".into()),
       Table::ShiftProblemNote      => return Err("implemented previously in ShiftNote enum varient".into()),
       Table::Undefined             => return Err("undefined table".into())
   }
   Ok(())
}

async fn update(app_state : &AppState, target_id : Uuid,table : Table,_other_id : Option<Uuid>) -> Result<(),Box<dyn Error>>{
   match table {
       Table::Employee              => {
           let employee = api::employee(app_state, target_id).await?;
           employee::update(&app_state.pool, employee).await?
       },
       Table::Problem               => {
           let problem = api::problem(app_state, target_id).await?;
           problem::update(&app_state.pool, problem).await?;
       },
       Table::SparePart             => {
           let part = api::spare_part(app_state, target_id).await?;
           spare_part::update(&app_state.pool, part).await?;
       },
       Table::Department            => {
           let dep = api::department(app_state, target_id).await?;
           department::update(&app_state.pool, dep).await?;
       },
       Table::Machine               => {
           let mac = api::machine(app_state, target_id).await?;
           machine::update(&app_state.pool, mac).await?;
       },
       Table::ShiftNote             => {
           let note = api::note(app_state, target_id).await?;
           note::update(&app_state.pool, Note{id : note.id, content : note.content}).await?;
       },
       Table::ShiftProblem          => {
           let sp = api::shift_problem(app_state, target_id).await?;
           shift_problem::update(&app_state.pool, sp).await?;
       },
       Table::DepartmentShift       => return Err("department shift cant't be updated".into()),
       Table::ShiftProblemProblem   => return Err("relations cant't be updated".into()),
       Table::ShiftProblemSparePart => return Err("relations cant't be updated".into()),
       Table::ShiftProblemNote      => return Err("implemented previously in ShiftNote enum varient".into()),
       Table::Shift                 => return Err("shift can not be updated".into()),
       Table::Undefined             => return Err("undefined table".into())
   }
   Ok(())
}
