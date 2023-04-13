use std::error::Error;

use sqlx::{query, Pool, Sqlite};

use rec::model::note::{DbNote, Note};
use uuid::Uuid;

pub async fn save_to_shift_problem(
    pool: &Pool<Sqlite>,
    note: DbNote<Uuid>,
) -> Result<(), Box<dyn Error>> {
    let DbNote {
        shift_id: _,
        id,
        shift_problem_id,
        content,
    } = note.string_to_client();
    match shift_problem_id {
        Some(shift_problem_id) => {
            let row = query!(
                "
        INSERT INTO note(
            id,
            shift_problem_id,
            content)
        VALUES($1,$2,$3) ON CONFLICT (id) DO NOTHING;",
                id,
                shift_problem_id,
                content
            )
            .execute(pool);
            return match row.await {
                Ok(_) => Ok(()),
                Err(err) => Err(err.into()),
            };
        }
        None => return Err("not qualified params".to_owned().into()),
    };
}

pub async fn save_to_shift(pool: &Pool<Sqlite>, note: DbNote<Uuid>) -> Result<(), Box<dyn Error>> {
    let DbNote {
        id,
        shift_id,
        shift_problem_id: _,
        content,
    } = note.string_to_client();
    match shift_id {
        Some(shift_id) => {
            let row = query!(
                "
        INSERT INTO note(
            id,
            shift_id,
            content)
        VALUES($1,$2,$3) ON CONFLICT (id) DO NOTHING;",
                id,
                shift_id,
                content
            )
            .execute(pool);
            return match row.await {
                Ok(_) => Ok(()),
                Err(err) => Err(err.into()),
            };
        }
        None => return Err("not qualified params".to_owned().into()),
    };
}

pub async fn update(pool: &Pool<Sqlite>, note: Note<Uuid>) -> Result<(), Box<dyn Error>> {
    let Note { id, content } = note.string_to_client();
    let row = query!(
        "
    UPDATE note SET content = $2 WHERE id =$1;",
        id,
        content
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete(pool: &Pool<Sqlite>, id: Uuid) -> Result<(), Box<dyn Error>> {
    let id = id.to_string();
    let row = query!(
        "
    DELETE FROM note WHERE id = $1",
        id
    )
    .execute(pool);
    match row.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
