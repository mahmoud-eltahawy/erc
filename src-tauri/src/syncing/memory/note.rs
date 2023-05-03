use std::error::Error;

use sqlx::{query, Pool, Sqlite};

use rec::model::note::{Note, ShiftNote};
use uuid::Uuid;

pub async fn save_to_shift_problem(pool: &Pool<Sqlite>, note: &Note) -> Result<(), Box<dyn Error>> {
    let Note { id, content } = note;
    let id = id.to_string();
    let row = query!(
        "
    INSERT INTO shift_problem_note(
    id,
    content)
    VALUES($1,$2) ON CONFLICT (id) DO NOTHING;",
        id,
        content
    )
    .execute(pool);
    return match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    };
}

pub async fn save_to_shift(pool: &Pool<Sqlite>, note: &ShiftNote) -> Result<(), Box<dyn Error>> {
    let ShiftNote {
        id,
        shift_id,
        writer_id,
        content,
    } = note;
    let id = id.to_string();
    let shift_id = shift_id.to_string();
    let writer_id = writer_id.to_string();
    let row = query!(
        "
    INSERT INTO shift_note(
    id,
    shift_id,
    writer_id,
    content)
    VALUES($1,$2,$3,$4) ON CONFLICT (id) DO NOTHING;",
        id,
        shift_id,
        writer_id,
        content
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
