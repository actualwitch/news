use crate::{
    api::*,
    model::{ApiError, Story, StoryGetArgs},
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::use_params,
    ParamSegment, StaticSegment,
};
use url::Url;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="icon" type="image/png" href="/favicon.png" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/news.css" />
        <Title text="Lambda Function" />
        <Router>
            <header>
                <span class="lambda-home".to_string()>
                    <A href="/">
                        <span class="lambda-icon".to_string()>λ</span>
                        Lambda Function
                    </A>
                </span>
                <span class="spacer".to_string()></span>
                <A href="/story/create/">"New"</A>
            </header>
            <main>
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=StaticSegment("") view=StoryList />
                    <Route
                        path=(StaticSegment("story"), StaticSegment("create"))
                        view=StoryCreate
                    />
                    <Route path=(StaticSegment("story"), ParamSegment("id")) view=StoryDetail />
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
fn StoryDetail() -> impl IntoView {
    let query = use_params::<StoryGetArgs>();
    let id = move || query.with(|q| q.as_ref().map(|q| q.id).map_err(|_| ApiError::InvalidData));
    let story = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => story_get(id).await.map_err(|_| ApiError::NotFound),
        }
    });
    // if (story_foo.is_none()) {
    //     return view! { <p>"Story not found."</p> };
    // }
    // let title = move || {
    //     story.get()
    // };
    view! {
        <Suspense fallback=|| view! { <p>"Loading stories..."</p> }>
            <h2>{move || story.get().map(|s| s.unwrap().title.clone()).unwrap_or_default()}</h2>
            <p></p>
        </Suspense>
    }
}

#[component]
fn StoryCreate() -> impl IntoView {
    let submit = ServerAction::<StoryCreate>::new();

    view! {
        <ActionForm action=submit>
            <header>New Story</header>
            <label>
                <span>Title</span>
                <input type="text" name="story[title]" />
            </label>
            <label>
                <span>Text</span>
                <textarea name="story[text]"></textarea>
            </label>
            <label>
                <span>URL</span>
                <input type="text" name="story[url]" />
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
                {domain
                    .is_some()
                    .then(|| {
                        view! {
                            <span>" → "</span>
                            <A href=format!("/by-domain/{}", domain.clone().unwrap())>{domain}</A>
                        }
                    })}
            </p>
        </li>
    }
}
