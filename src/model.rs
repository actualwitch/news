use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;

#[derive(Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Story {
    pub id: i32,
    pub title: String,
    #[builder(default, setter(strip_option))]
    pub text: Option<String>,
    #[builder(default, setter(strip_option))]
    pub url: Option<Url>,
    pub created_at: DateTime<FixedOffset>,
    #[builder(default, setter(strip_option))]
    pub updated_at: Option<DateTime<FixedOffset>>,
    pub author_id: i32,
}


#[derive(Clone, Serialize, Deserialize, TypedBuilder)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: Option<DateTime<FixedOffset>>,
}