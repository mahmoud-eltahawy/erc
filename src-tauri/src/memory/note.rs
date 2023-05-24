use std::str::FromStr;

use sqlx::{query, Pool, Sqlite};

use rec::model::note::{Note, ShiftNote};
use uuid::Uuid;

pub async fn fetch_shift_note_by_id(pool: &Pool<Sqlite>, id: Uuid) -> Option<ShiftNote> {
    let id = id.to_string();
    let row = query!(
        r#"
    SELECT id,shift_id,content FROM shift_note WHERE id = $1;"#,
        id
    )
    .fetch_one(pool);
    match row.await {
        Ok(record) => match (Uuid::from_str(&record.id), Uuid::from_str(&record.shift_id)) {
            (Ok(id), Ok(shift_id)) => Some(ShiftNote {
                id,
                shift_id,
                content: record.content,
            }),
            _ => None,
        },
        Err(_) => None,
    }
}

pub async fn fetch_shift_problem_note_by_id(pool: &Pool<Sqlite>, id: Uuid) -> Option<Note> {
    let row = query!(
        r#"
    SELECT * FROM shift_problem_note WHERE id = $1;"#,
        id
    )
    .fetch_one(pool);
    match row.await {
        Ok(record) => match Uuid::from_str(&record.id) {
            Ok(id) => Some(Note {
                id,
                content: record.content,
            }),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

pub async fn fetch_shift_notes_ids_by_shift_id(pool: &Pool<Sqlite>, id: Uuid) -> Option<Vec<Uuid>> {
    let id = id.to_string();
    let row = query!(
        r#"
      SELECT id FROM shift_note WHERE shift_id = $1;
    "#,
        id
    )
    .fetch_all(pool);
    match row.await {
        Ok(records) => Some(
            records
                .into_iter()
                .filter_map(|record| Uuid::from_str(&record.id).ok())
                .collect(),
        ),
        Err(_) => None,
    }
}

pub async fn fetch_shift_problem_note(
    pool: &Pool<Sqlite>,
    id: &Uuid,
) -> Result<String, Box<dyn std::error::Error>> {
    let id = id.to_string();
    let record = query!(
        r#"
    SELECT content FROM shift_problem_note WHERE id = $1;"#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(record.content)
}
