use crate::model::{Story, StoryCreateArgs};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::prelude::*;
    use sqlx::PgPool;

    pub fn pool() -> Result<PgPool, ServerFnError> {
        use_context::<PgPool>().ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
    }
}

#[server]
pub async fn get_stories() -> Result<Vec<Story>, ServerFnError> {
    use self::ssr::pool;
    use sqlx::query_as;

    let pool = pool()?;

    let stories = query_as!(Story, r#"SELECT * FROM stories"#)
        .fetch_all(&pool)
        .await?;

    Ok(stories)
}

#[server]
pub async fn story_create(story: StoryCreateArgs) -> Result<Story, ServerFnError> {
    use self::ssr::pool;
    use chrono::Local;
    use sqlx::query_as;

    let pool = pool()?;
    let timestamp = Local::now();

    if story.title.is_empty() {
        return Err(ServerFnError::ServerError("Title is required.".into()));
    }

    if story.text.clone().is_none_or(|text| text.is_empty()) && story.url.clone().is_none_or(|url| url.is_empty()) {
        return Err(ServerFnError::ServerError("Text or URL is required.".into()));
    }

    let result = query_as!(Story, r#"INSERT INTO stories (title, text, url, author_id, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"#, story.title, story.text, story.url, 1, timestamp.into())
        .fetch_one(&pool)
        .await?;

    Ok(result)
}

#[server]
pub async fn story_get(id: i32) -> Result<Story, ServerFnError> {
    use self::ssr::pool;
    use sqlx::query_as;

    let pool = pool()?;

    let story = query_as!(Story, r#"SELECT * FROM stories WHERE id = $1"#, id)
        .fetch_one(&pool)
        .await?;

    Ok(story)
}
