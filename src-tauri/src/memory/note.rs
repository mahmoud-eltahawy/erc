use sqlx::{query_as, Pool, Sqlite};

use rec::model::note::{ClientDbNote, ClientNote};

pub async fn fetch_note_by_id(pool : &Pool<Sqlite>,
                        id : String) -> Option<ClientDbNote> {
  let row = query_as!(ClientDbNote,r#"
    SELECT id,shift_id,shift_problem_id,content FROM note WHERE id = $1;"#,id)
    .fetch_one(pool);
  match row.await {
    Ok(name) => Some(name),
    Err(_) => None
  }
}

pub async fn fetch_shift_problem_note(pool : &Pool<Sqlite>,
                        id : &String) -> Option<ClientNote> {
  let row = query_as!(ClientNote,r#"
    SELECT id,content FROM note WHERE shift_problem_id = $1;"#,id)
    .fetch_one(pool);
  match row.await {
    Ok(name) => Some(name),
    Err(_) => None
  }
}
