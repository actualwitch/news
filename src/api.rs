use crate::model::{Story, StoryCreateArgs, StoryListItem};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::prelude::*;
    use sqlx::PgPool;

    pub fn pool() -> Result<PgPool, ServerFnError> {
        use_context::<PgPool>().ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
    }
}

const PAGE_SIZE: i64 = 30;

#[server]
pub async fn get_stories(page: Option<i64>) -> Result<Vec<StoryListItem>, ServerFnError> {
    use self::ssr::pool;
    use sqlx::query_as;

    let pool = pool()?;

    let offset: i64 = page.unwrap_or(0) * PAGE_SIZE;

    let stories = query_as!(
        StoryListItem,
        r#"
            SELECT 
                s.id, 
                s.title, 
                s.text, 
                s.url, 
                s.created_at, 
                u.display_name as author_name,
                0 as rating,
                (SELECT COUNT(*)::integer FROM comments c WHERE c.story_id = s.id) as comment_count
            FROM 
                stories s
            JOIN 
                users u ON s.author_id = u.id
            ORDER BY 
                s.created_at DESC
            LIMIT $1 
            OFFSET $2
        "#,
        PAGE_SIZE,
        offset,
    )
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

    if story.text.clone().is_none_or(|text| text.is_empty())
        && story.url.clone().is_none_or(|url| url.is_empty())
    {
        return Err(ServerFnError::ServerError(
            "Text or URL is required.".into(),
        ));
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

#[server]
pub async fn comment_create(story_id: i32, parent_id: Option<i32>, text: String) -> Result<(), ServerFnError> {
    use self::ssr::pool;
    use chrono::Local;
    use sqlx::query;

    let pool = pool()?;
    let timestamp = Local::now();

    query!(
        r#"INSERT INTO comments (story_id, parent_id, text, author_id, created_at) VALUES ($1, $2, $3, $4, $5)"#,
        story_id,
        parent_id,
        text,
        1,
        timestamp.into()
    )
    .execute(&pool)
    .await?;

    Ok(())
}