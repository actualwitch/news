use chrono::{DateTime, FixedOffset};
use leptos::{error, Params};
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use typed_builder::TypedBuilder;
use url::Url;
use validator::{Validate, ValidationError};

#[cfg(feature = "ssr")]
pub mod ssr {
    use axum::extract::FromRef;
    use leptos::prelude::LeptosOptions;
    use leptos_axum::AxumRouteListing;
    use sqlx::PgPool;

    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub pool: PgPool,
        pub routes: Vec<AxumRouteListing>,
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LambdaError {
    #[error("No adjoint functor exists.")]
    NotFound,
    #[error("Combinator crashed—please contact Curry or Howard.")]
    InternalServerError,
    #[error("This morphism is not well-defined: {0}.")]
    InvalidData(String),
    #[error("Non-terminating reduction.")]
    Timeout,
    #[error("Type inhabitance failure.")]
    AuthError,
    #[error("Coherence conditions not satisfied: {0}.")]
    ValidationError(String),
}

#[derive(Clone, Serialize, Deserialize, TypedBuilder, Debug)]
pub struct Story {
    pub id: i32,
    pub title: String,
    #[builder(default, setter(strip_option))]
    pub text: Option<String>,
    #[builder(default, setter(strip_option))]
    pub url: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub author_id: i32,
}

#[derive(Clone, Serialize, Deserialize, TypedBuilder, Debug)]
pub struct StoryListItem {
    pub id: i32,
    pub title: String,
    #[builder(default, setter(strip_option))]
    pub text: Option<String>,
    #[builder(default, setter(strip_option))]
    pub url: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub author_name: String,
    pub rating: Option<i32>,
    pub comment_count: Option<i32>,
}

impl Into<Story> for StoryListItem {
    fn into(self) -> Story {
        Story {
            id: self.id,
            title: self.title,
            text: self.text,
            url: self.url,
            created_at: self.created_at,
            author_id: 0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, TypedBuilder, Debug, Validate)]
#[validate(schema(function = "validate_story_create_args"))]
pub struct StoryCreateArgs {
    #[validate(length(min = 1, message = "is empty"))]
    pub title: String,
    #[builder(default, setter(strip_option))]
    pub text: Option<String>,
    #[builder(default, setter(strip_option))]
    #[validate(url)]
    pub url: Option<String>,
}

fn validate_story_create_args(story: &&StoryCreateArgs) -> Result<(), ValidationError> {
    if story.text.as_deref().map_or(true, str::is_empty)
        && story.url.as_deref().map_or(true, str::is_empty)
    {
        return Err(ValidationError::new("Text or URL is required."));
    }
    Ok(())
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Params, TypedBuilder, Debug)]
pub struct StoryGetArgs {
    pub id: i32,
}

#[derive(Clone, Serialize, Deserialize, TypedBuilder)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, TypedBuilder)]
pub struct Comment {
    pub id: i32,
    pub text: String,
    pub parent_id: Option<i32>,
    pub story_id: i32,
    pub created_at: DateTime<FixedOffset>,
    pub author_name: String,
}

#[derive(Clone, Serialize, Deserialize, TypedBuilder, Debug)]
pub struct CommentCreateArgs {
    pub text: String,
    pub parent_id: Option<i32>,
    pub story_id: i32,
}
