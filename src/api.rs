use crate::model::{Comment, CommentCreateArgs, Story, StoryCreateArgs, StoryListItem};
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
pub async fn story_list(page: Option<i64>) -> Result<Vec<StoryListItem>, ServerFnError> {
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
pub async fn get_story_page_count() -> Result<i64, ServerFnError> {
    use self::ssr::pool;
    use sqlx::query;

    let pool = pool()?;

    let count = query!("SELECT COUNT(*) FROM stories")
        .fetch_one(&pool)
        .await?
        .count;

    let pages = count.map(|count| count / PAGE_SIZE + 1).unwrap_or(0);

    Ok(pages)
}

#[server]
pub async fn comment_create(comment: CommentCreateArgs) -> Result<(), ServerFnError> {
    use self::ssr::pool;
    use chrono::Local;
    use sqlx::query;

    let pool = pool()?;
    let timestamp = Local::now();

    query!(
        r#"INSERT INTO comments (story_id, parent_id, text, author_id, created_at) VALUES ($1, $2, $3, $4, $5)"#,
        comment.story_id,
        comment.parent_id,
        comment.text,
        1,
        timestamp.into()
    )
    .execute(&pool)
    .await?;

    Ok(())
}

#[server]
pub async fn comment_list(story_id: i32) -> Result<Vec<Comment>, ServerFnError> {
    use self::ssr::pool;
    use sqlx::query_as;

    let pool = pool()?;

    let comments = query_as!(
        Comment,
        r#"SELECT * FROM comments WHERE story_id = $1 ORDER BY created_at DESC"#,
        story_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(comments)
}
