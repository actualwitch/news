use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;

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
pub struct StoryCreateArgs {
    pub title: String,
    #[builder(default, setter(strip_option))]
    pub text: Option<String>,
    #[builder(default, setter(strip_option))]
    pub url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, TypedBuilder)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<FixedOffset>,
}
