use std::error::Error;

use sqlx::{query, Pool, Sqlite};

use rec::model::note::{Note, ShiftNote};
use uuid::Uuid;

use crate::syncing::Env;

pub async fn save_to_shift_problem(
    pool: &Pool<Sqlite>,
    note: &Note,
    env: Env,
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    let Note { id, content } = note;
    let id = id.to_string();

    let updater_id = updater_id.to_string();
    let time_stamp = serde_json::json!(time_stamp).to_string();
    let row = query!(
        "
    INSERT INTO shift_problem_note(
    id,content,updater_id,time_stamp)
    VALUES($1,$2,$3,$4) ON CONFLICT (id) DO NOTHING;",
        id,
        content,
        updater_id,
        time_stamp
    )
    .execute(pool);
    return match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    };
}

pub async fn save_to_shift(
    pool: &Pool<Sqlite>,
    note: &ShiftNote,
    env: Env,
) -> Result<(), Box<dyn Error>> {
    let ShiftNote {
        id,
        shift_id,
        content,
    } = note;
    let (updater_id, time_stamp) = env;
    let id = id.to_string();
    let shift_id = shift_id.to_string();
    let updater_id = updater_id.to_string();
    let time_stamp = serde_json::json!(time_stamp).to_string();
    let row = query!(
        "
    INSERT INTO shift_note(
    id,
    shift_id,
    content,
    updater_id,
    time_stamp)
    VALUES($1,$2,$3,$4,$5) ON CONFLICT (id) DO NOTHING;",
        id,
        shift_id,
        content,
        updater_id,
        time_stamp
    )
    .execute(pool);
    return match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    };
}

pub async fn update_shift_note(pool: &Pool<Sqlite>, note: &Note) -> Result<(), Box<dyn Error>> {
    let Note { id, content } = note;
    let id = id.to_string();
    let row = query!(
        "
    UPDATE shift_note SET content = $2 WHERE id =$1;",
        id,
        content
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn update_shift_problem_note(
    pool: &Pool<Sqlite>,
    note: &Note,
) -> Result<(), Box<dyn Error>> {
    let Note { id, content } = note;
    let id = id.to_string();
    let row = query!(
        "
    UPDATE shift_problem_note SET content = $2 WHERE id =$1;",
        id,
        content
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_shift_note(pool: &Pool<Sqlite>, id: Uuid) -> Result<(), Box<dyn Error>> {
    let id = id.to_string();
    let row = query!(
        "
    DELETE FROM shift_note WHERE id = $1",
        id
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_shift_problem_note(
    pool: &Pool<Sqlite>,
    id: Uuid,
) -> Result<(), Box<dyn Error>> {
    let id = id.to_string();
    let row = query!(
        "
    DELETE FROM shift_problem_note WHERE id = $1",
        id
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
