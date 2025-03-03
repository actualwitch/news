use crate::{
    api::*,
    model::{Story, StoryGetArgs, StoryListItem},
};
use leptos::{either::Either, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::{use_navigate, use_params},
    ParamSegment, SsrMode, StaticSegment,
};
use opentelemetry_sdk::error;
use url::Url;

const LAMBDA_FUNCTION: &str = "Lambda Function";

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
        <Router>
            <header>
                <span class="lambda-home".to_string()>
                    <A href="/">
                        <span class="lambda-icon".to_string()>λ</span>
                        {LAMBDA_FUNCTION}
                    </A>
                </span>
                <span class="spacer".to_string()></span>
                <A href="/story/create">"New"</A>
            </header>
            <main>
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=StaticSegment("") view=StoryList />
                    <Route
                        path=(StaticSegment("story"), StaticSegment("create"))
                        view=StoryCreate
                    />
                    <Route
                        path=(StaticSegment("story"), ParamSegment("id"))
                        view=StoryDetail
                        ssr=SsrMode::Async
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn StoryList() -> impl IntoView {
    let (page, set_page) = signal(0 as i64);
    let stories_resource = Resource::new(move || page.get(), |page| get_stories(Some(page)));
    let stories = move || {
        stories_resource
            .get()
            .map(|n| n.unwrap_or_default())
            .unwrap_or_default()
    };
    view! {
        <Title text=format!("{} :: {}", "Latest", LAMBDA_FUNCTION) />
        <Suspense fallback=|| view! { <p>"Loading stories..."</p> }>
            <ol class="stories".to_string()>
                <For each=stories key=|story| story.id let:story>
                    <StoryLink story=story />
                </For>
            </ol>
        </Suspense>
    }
}

#[component]
fn StoryDetail() -> impl IntoView {
    let query = use_params::<StoryGetArgs>();
    let id = move || query.with(|q| q.clone().map(|q| q.id));
    let story = Resource::new(
        id,
        |id| async move { story_get(id.unwrap_or_default()).await },
    );

    Suspense(
        SuspenseProps::builder()
            .fallback(|| "Loading...")
            .children(ToChildren::to_children(move || {
                Suspend::new(async move {
                    match story.await.clone() {
                        Err(_) => Either::Right(
                            view! { <p class="error".to_string()>"Story not found."</p> },
                        ),
                        Ok(Story { title, text, .. }) => Either::Left(view! {
                            <Title text=format!("{} :: {}", title, LAMBDA_FUNCTION) />
                            <h2>{title}</h2>
                            <p>{text}</p>
                        }),
                    }
                })
            }))
            .build(),
    )
}

#[component]
fn StoryCreate() -> impl IntoView {
    let submit = ServerAction::<StoryCreate>::new();

    let value = submit.value();

    let navigate = use_navigate();

    Effect::watch(
        move || submit.value().get(),
        move |story, _, _| match story {
            Some(Ok(story)) => {
                navigate(format!("/story/{}", story.id).as_str(), Default::default());
            }
            _ => {}
        },
        false,
    );

    let success = move || match value.get() {
        Some(Ok(Story { id, .. })) => Some(view! {
            <p class="success">
                <A href=format!("/story/{}", id)>"Story created."</A>
            </p>
        }),
        _ => None,
    };

    let error = move || match value.get() {
        Some(Err(e)) => Some(view! {
            <p class="error"
                .to_string()>
                {match e {
                    ServerFnError::ServerError(e) => e.to_string(),
                    _ => "An error occurred.".to_string(),
                }}
            </p>
        }),
        _ => None,
    };

    view! {
        <Title text=format!("{} :: {}", "New", LAMBDA_FUNCTION) />
        <ActionForm action=submit>
            {success} {error} <header>New Story</header> <label>
                <span>Title</span>
                <input type="text" name="story[title]" />
            </label> <label>
                <span>Text</span>
                <textarea name="story[text]"></textarea>
            </label> <label>
                <span>URL</span>
                <input type="text" name="story[url]" />
            </label> <button type="submit">"Create"</button>
        </ActionForm>
    }
}

#[component]
fn StoryLink(#[prop(into)] story: StoryListItem) -> impl IntoView {
    let url = format!("/story/{0}", story.id);
    let profile_url = format!("/user/{0}", story.author_name);
    let timestamp = story.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
    let domain: Option<String> = try {
        let url = story.url.as_ref()?;
        let url = Url::parse(url).ok()?;
        let domain = url.domain()?;
        domain.into()
    };
    view! {
        <li>
            <p>
                <A href=url.clone()>{story.title}</A>
                {domain
                    .is_some()
                    .then(|| {
                        view! {
                            <span>" → "</span>
                            <A href=format!("/by-domain/{}", domain.clone().unwrap())>{domain}</A>
                        }
                    })}
            </p>
            <p class="meta".to_string()>
                <A href=url>{story.comment_count}comments</A>
                <span>submitted by <A href=profile_url>{story.author_name}</A></span>
                <span title=story.created_at.to_string()>{timestamp}</span>
            </p>
        </li>
    }
}
