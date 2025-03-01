use crate::{api::*, model::Story};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};
use url::Url;

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
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/news.css"/>
        <Title text="Lambda Function"/>
        <Router>
            <header>
                <span class="lambda-home".to_string()><A href="/"><span class="lambda-icon".to_string()>λ</span>Lambda Function</A></span>
                <span class="spacer".to_string()></span>
                <A href="/story/create/">"New"</A>
            </header>
            <main>
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=StaticSegment("") view=StoryList/>
                    <Route path=(StaticSegment("story"), StaticSegment("create")) view=StoryCreate/>
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

#[component]
fn StoryDetail(id: i32) -> impl IntoView {
    let story = Resource::new(move || id, |id| get_story(id));
    let title = story
        .get()
        .map(|s| s.expect("expected story").title.clone())
        .unwrap_or_default();
    view! {
        <div>
            <h1>{title}</h1>
            <p>{story.get().map(|s| s.expect("expected story").text.clone()).unwrap_or_default()}</p>
        </div>
    }
}

#[component]
fn StoryCreate() -> impl IntoView {
    let submit = ServerAction::<CreateStory>::new();

    view! {
        <ActionForm action=submit>
            <header>New Story</header>
            <label>
                <span>Title</span>
                <input type="text" name="story[title]"/>
            </label>
            <label>
                <span>Text</span>
                <textarea name="story[text]"></textarea>
            </label>
            <label>
                <span>URL</span>
                <input type="text" name="story[url]"/>
            </label>
            <button type="submit">"Create"</button>
        </ActionForm>
    }
}

#[component]
fn StoryLink(story: Story) -> impl IntoView {
    let url = format!("/story/{0}/", story.id);
    let domain: Option<String> = try {
        let url = story.url.as_ref()?;
        let url = Url::parse(url).ok()?;
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
