use sqlx::{query, query_as, Pool, Sqlite};

use rec::model::note::{DbNote, Note};

pub async fn fetch_note_by_id(pool: &Pool<Sqlite>, id: String) -> Option<DbNote<String>> {
    let row = query_as!(
        DbNote,
        r#"
    SELECT id,shift_id,shift_problem_id,content FROM note WHERE id = $1;"#,
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
) -> Option<Vec<String>> {
    let row = query!(
        r#"
      SELECT content FROM note WHERE shift_id = $1;
    "#,
        id
    )
    .fetch_all(pool);
    match row.await {
        Ok(r) => Some(r.into_iter().map(|r| r.content).collect()),
        Err(_) => None,
    }
}

pub async fn fetch_shift_problem_note(pool: &Pool<Sqlite>, id: &String) -> Option<Note<String>> {
    let row = query_as!(
        Note,
        r#"
    SELECT id,content FROM note WHERE shift_problem_id = $1;"#,
        id
    )
    .fetch_one(pool);
    match row.await {
        Ok(name) => Some(name),
        Err(_) => None,
    }
}
