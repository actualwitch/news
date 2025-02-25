use chrono::{DateTime, FixedOffset};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;

#[derive(Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
pub struct Story {
    title: String,
    #[builder(default, setter(strip_option))]
    text: Option<String>,
    #[builder(default, setter(strip_option))]
    url: Option<Url>,
    created_at: DateTime<FixedOffset>,
    #[builder(default, setter(strip_option))]
    updated_at: Option<DateTime<FixedOffset>>,
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" type="image/png" href="favicon.png" />
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/news.css"/>

        // sets the document title
        <Title text="Lambda Function"/>

        <header>
            <span class="lambda-icon".to_string()>λ</span>
            <span>Lambda Function</span>
        </header>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

// /// Renders the home page of your application.
// #[component]
// fn HomePage() -> impl IntoView {
//     // Creates a reactive value to update the button
//     let count = RwSignal::new(0);
//     let on_click = move |_| *count.write() += 1;

//     view! {
//         <h1>"Welcome to Leptos!"</h1>
//         <button on:click=on_click>"Click Me: " {count}</button>
//     }
// }

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let async_data = Resource::new(|| 0, move |_| get_news());
    view! {
        <div />
    }
}

#[server]
pub async fn get_news() -> Result<Vec<Story>, ServerFnError> {
    let timestamp = Local::now();
    Ok(vec![Story::builder()
        .title("🦀 React is dead 🦀".to_string())
        .created_at(timestamp.into())
        .url(Url::parse("https://www.rust-lang.org/")?)
        .build()])
}
