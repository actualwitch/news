use crate::{
    constants::PAGE_SIZE,
    model::{Comment, CommentCreateArgs, LambdaError, Story, StoryCreateArgs, StoryListItem},
};
use leptos::prelude::*;
use validator::Validate;

#[server]
pub async fn story_list(page: Option<i64>) -> Result<Vec<StoryListItem>, ServerFnError> {
    use crate::server::pool;
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
    use crate::server::pool;
    use chrono::Local;
    use sqlx::query_as;

    let pool = pool()?;
    let timestamp = Local::now();

    story.validate().map_err(|err| LambdaError::ValidationError(err.to_string()))?;

    let result = query_as!(Story, r#"INSERT INTO stories (title, text, url, author_id, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"#, story.title, story.text, story.url, 1, timestamp.into())
        .fetch_one(&pool)
        .await?;

    Ok(result)
}

#[server]
pub async fn story_get(id: i32) -> Result<Story, ServerFnError> {
    use crate::server::pool;
    use sqlx::query_as;

    let pool = pool()?;

    let story = query_as!(Story, r#"SELECT * FROM stories WHERE id = $1"#, id)
        .fetch_one(&pool)
        .await?;

    Ok(story)
}

#[server]
pub async fn get_story_page_count() -> Result<i64, ServerFnError> {
    use crate::server::pool;
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
    use crate::server::pool;
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
    use crate::server::pool;
    use sqlx::query_as;

    let pool = pool()?;

    let comments = query_as!(
        Comment,
        r#"
            SELECT 
                c.id,
                c.text,
                c.parent_id,
                c.story_id,
                c.created_at,
                u.display_name as author_name
            FROM comments c
            JOIN 
                users u ON c.author_id = u.id
            WHERE story_id = $1
            ORDER BY created_at DESC"#,
        story_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(comments)
}

#[server]
pub async fn comment_with_parents(comment_id: i32) -> Result<Vec<Comment>, ServerFnError> {
    use crate::server::{pool, row_to_comment};
    use chrono::{DateTime, FixedOffset, Utc};
    use sqlx::{postgres::PgRow, query, Row};

    let pool = pool()?;

    let rows = query(
        r#"
        WITH RECURSIVE comment_hierarchy AS (
            -- Base case: the starting comment
            SELECT 
                c.id,
                c.text,
                c.parent_id,
                c.story_id,
                c.created_at,
                u.display_name as author_name
            FROM 
                comments c
            JOIN 
                users u ON c.author_id = u.id
            WHERE 
                c.id = $1
            
            UNION ALL
            
            -- Recursive case: parent comments
            SELECT 
                c.id,
                c.text,
                c.parent_id,
                c.story_id,
                c.created_at,
                u.display_name as author_name
            FROM 
                comments c
            JOIN 
                users u ON c.author_id = u.id
            JOIN 
                comment_hierarchy ch ON c.id = ch.parent_id
        )
        SELECT * FROM comment_hierarchy
        ORDER BY created_at ASC -- Order from oldest to newest
        "#,
    )
    .bind(comment_id)
    .fetch_all(&pool)
    .await?;

    let comments = rows
        .into_iter()
        .map(row_to_comment)
        .collect();

    Ok(comments)
}