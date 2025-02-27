
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};
use url::Url;
use crate::model::Story;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" type="image/png" href="/favicon.png" />
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
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=StaticSegment("") view=StoryList/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn StoryList() -> impl IntoView {
    let (page, set_page) = signal(0);
    let stories_resource = Resource::new(move || page.get(), |_| get_stories());
    let stories = move || {
        stories_resource
            .get()
            .map(|n| n.unwrap_or_default())
            .unwrap_or_default()
    };
    view! {
        <Suspense fallback=|| view! { <p>"Loading stories..."</p> }>
            <ol>
                <For each=stories key=|story| story.id let:story>
                    <StoryLink story />
                </For>
            </ol>
        </Suspense>
    }
}

// #[component]
// fn StoryDetail(id: i32) -> impl IntoView {
//     let story = Resource::new(move || id, |id| get_story(id));
//     let title = story.get().map(|s| s.title.clone()).unwrap_or_default();
//     view! {
//         <div>
//             <h1>{title}</h1>
//             <p>{story.get().map(|s| s.text.clone()).unwrap_or_default()}</p>
//         </div>
//     }
// }

#[component]
fn StoryLink(story: Story) -> impl IntoView {
    let url = format!("/story/{0}/", story.id);
    let domain: Option<String> = try {
        let url = story.url.as_ref()?;
        let domain = url.domain()?;
        domain.into()
    };
    view! {
        <li>
            <p>
                <A href=url>{story.title}</A>
                {domain.is_some().then(|| {view! {
                    <span>" → "</span>
                    <A href=format! ("/by-domain/{}", domain.clone().unwrap())>{domain}</A>
                }})}
            </p>
        </li>
    }
}

#[server]
pub async fn get_stories() -> Result<Vec<Story>, ServerFnError> {
    let timestamp = chrono::Local::now();
    Ok(vec![
        Story::builder()
            .id(0)
            .author_id(0)
            .title("🦀 React is dead 🦀".to_string())
            .created_at(timestamp.into())
            .url(Url::parse("https://www.leptos.dev/")?)
            .build(),
        Story::builder()
            .id(1)
            .author_id(0)
            .title("Thoughtful post".to_string())
            .created_at(timestamp.into())
            .text("It contains thoughts.".to_string())
            .build(),
    ])
}

#[server]
pub async fn get_story(id: i32) -> Result<Story, ServerFnError> {
    let timestamp = chrono::Local::now();
    Ok(Story::builder()
        .id(id)
        .author_id(0)
        .title("🦀 React is dead 🦀".to_string())
        .created_at(timestamp.into())
        .url(Url::parse("https://www.leptos.dev/")?)
        .build())
}