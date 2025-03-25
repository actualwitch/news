
use crate::model::Comment;
use chrono::{DateTime, Local};
use leptos::prelude::*;
use sqlx::PgPool;
use sqlx::postgres::PgRow;
use sqlx::Row;

pub fn pool() -> Result<PgPool, ServerFnError> {
    use_context::<PgPool>().ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
}

pub fn row_to_comment(row: PgRow) -> Comment {
    Comment {
        id: row.get("id"),
        text: row.get("text"),
        parent_id: row.get("parent_id"),
        story_id: row.get("story_id"),
        created_at: row.get::<DateTime<Local>, _>("created_at").into(),
        author_name: row.get("author_name"),
    }
}