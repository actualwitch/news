use chrono::{DateTime, FixedOffset};
use leptos::Params;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;
use thiserror::Error;

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


#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiError {
    #[error("Not found.")]
    NotFound,
    #[error("Unauthorized.")]
    Unauthorized,
    #[error("Internal server error.")]
    InternalServerError,
    #[error("Invalid data.")]
    InvalidData,
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

#[derive(Clone, Serialize, Deserialize, TypedBuilder, Debug)]
pub struct StoryCreateArgs {
    pub title: String,
    #[builder(default, setter(strip_option))]
    pub text: Option<String>,
    #[builder(default, setter(strip_option))]
    pub url: Option<String>,
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
