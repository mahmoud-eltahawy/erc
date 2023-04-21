use std::str::FromStr;

use sqlx::{query, query_as, Pool, Sqlite};

use rec::model::note::{Note, ShiftNote};
use uuid::Uuid;

pub async fn fetch_shift_note_by_id(pool: &Pool<Sqlite>, id: String) -> Option<ShiftNote<String>> {
    let row = query_as!(
        ShiftNote,
        r#"
    SELECT * FROM shift_note WHERE id = $1;"#,
        id
    )
    .fetch_one(pool);
    match row.await {
        Ok(name) => Some(name),
        Err(_) => None,
    }
}

pub async fn fetch_shift_problem_note_by_id(
    pool: &Pool<Sqlite>,
    id: String,
) -> Option<Note<String>> {
    let row = query_as!(
        Note,
        r#"
    SELECT * FROM shift_problem_note WHERE id = $1;"#,
        id
    )
    .fetch_one(pool);
    match row.await {
        Ok(name) => Some(name),
        Err(_) => None,
    }
}

pub async fn fetch_notes_content_by_shift_id(
    pool: &Pool<Sqlite>,
    id: String,
) -> Option<Vec<(Uuid, String)>> {
    let row = query!(
        r#"
      SELECT writer_id,content FROM shift_note WHERE shift_id = $1;
    "#,
        id
    )
    .fetch_all(pool);
    match row.await {
        Ok(records) => Some(
            records
                .into_iter()
                .filter(|record| Uuid::from_str(&record.writer_id).is_ok())
                .map(|record| (Uuid::from_str(&record.writer_id).unwrap(), record.content))
                .collect(),
        ),
        Err(_) => None,
    }
}

pub async fn fetch_shift_problem_note(pool: &Pool<Sqlite>, id: &Uuid) -> Option<Note<String>> {
    let id = id.to_string();
    let row = query_as!(
        Note,
        r#"
    SELECT id,content FROM shift_problem_note WHERE id = $1;"#,
        id
    )
    .fetch_one(pool);
    match row.await {
        Ok(name) => Some(name),
        Err(_) => None,
    }
}
