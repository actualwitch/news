use crate::{
    api::*,
    model::{Comment, Story, StoryGetArgs, StoryListItem},
};
use chrono::{DateTime, FixedOffset, Local};
use chrono_humanize::HumanTime;
use leptos::{either::Either, logging::log, prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::{use_navigate, use_params},
    ParamSegment, SsrMode, StaticSegment,
};
use leptos_use::use_interval_fn;
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

fn provide_now() {
    let (now, set_now) = signal(Local::now());
    provide_context(now);
    use_interval_fn(
        move || {
            set_now(Local::now());
        },
        60_000,
    );
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_now();

    view! {
        <Stylesheet id="leptos" href="/pkg/news.css" />
        <Router>
            <header>
                <span class="lambda-home".to_string()>
                    <A href="/">
                        <span class="lambda-icon".to_string()>λ</span>
                        <span class="lambda-text".to_string()>{LAMBDA_FUNCTION}</span>
                    </A>
                </span>
                <span class="spacer".to_string()></span>
                <A href="/story/create">"New"</A>
            </header>
            <main>
                <Routes fallback=NotFound>
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
    let stories_resource = Resource::new(move || page.get(), |page| story_list(Some(page)));
    let stories = move || {
        stories_resource
            .get()
            .map(|n| n.unwrap_or_default())
            .unwrap_or_default()
    };
    view! {
        <Title text=format!("{} :: {}", "Latest", LAMBDA_FUNCTION) />
        <Suspense fallback=|| view! { <p>"Loading stories..."</p> }>
            <ol class="stories".to_string() start=0>
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
    let story_res = Resource::new(id, |id| async move {
        match id {
            Ok(id) => story_get(id).await,
            _ => Err(ServerFnError::ServerError("Story not found.".into())),
        }
    });
    let comments = Resource::new(id, |story_id| async move {
        match story_id {
            Ok(story_id) => comment_list(story_id).await.unwrap_or(vec![]),
            _ => vec![],
        }
    });

    view! {
        <Suspense fallback=|| {
            "Loading..."
        }>
            {move || {
                Suspend::new(async move {
                    let story = story_res.await;
                    comments.await;
                    view! {
                        {match story {
                            Ok(Story { title, text, id, .. }) => {
                                Either::Left(
                                    view! {
                                        <Title text=format!("{} :: {}", title, LAMBDA_FUNCTION) />
                                        <h2>{title}</h2>
                                        <p>{text}</p>
                                        <CommentCreate
                                            story_id=id
                                            on_submit=move || {
                                                comments.refetch();
                                            }
                                        />
                                    },
                                )
                            }
                            Err(_) => Either::Right(NotFound),
                        }}
                        <ol class="comments".to_string()>
                            <For
                                each=move || comments.get().unwrap().into_iter().enumerate()
                                key=|(_, comment)| comment.id
                                children=move |(index, _)| {
                                    Memo::new(move |_| { comments.get()?.get(index).cloned() })
                                        .get()
                                        .clone()
                                        .and_then(|comment| {
                                            view! { <CommentDetail comment=comment.clone() /> }.into()
                                        })
                                }
                            />
                        </ol>
                    }
                })
            }}
        </Suspense>
    }
}

#[component]
fn StoryCreate() -> impl IntoView {
    let navigate = use_navigate();

    let submit = ServerAction::<StoryCreate>::new();
    let value = submit.value();
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
fn RelativeTime(from: DateTime<FixedOffset>) -> impl IntoView {
    let now = use_context::<ReadSignal<DateTime<Local>>>().expect("now should be provided");
    let human_time = move || {
        let duration = from.signed_duration_since(now.get());
        HumanTime::from(duration).to_string()
    };
    view! {
        <span title=from.to_string()>{human_time}</span>
    }
}

#[component]
fn StoryLink(#[prop(into)] story: StoryListItem) -> impl IntoView {
    let url = format!("/story/{0}", story.id);

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
                <A href=url>{story.comment_count}" comments"</A>
                <span>submitted by <UserLink user_name=story.author_name /></span>
                <RelativeTime from=story.created_at />
            </p>
        </li>
    }
}

#[component]
fn UserLink(user_name: String) -> impl IntoView {
    let profile_url = format!("/user/{0}", user_name);
    view! { <A href=profile_url>{user_name}</A> }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text=format!("{} :: {}", "Not found", LAMBDA_FUNCTION) />
        <main class="error".to_string()>
            <header>404</header>
            <p>"Not found."</p>
        </main>
    }
}

#[component]
fn CommentCreate(
    #[prop(optional)] parent_id: Option<i32>,
    story_id: i32,
    on_submit: impl Fn() -> () + 'static,
) -> impl IntoView {
    let navigate = use_navigate();
    let submit = ServerAction::<CommentCreate>::new();
    let input_element: NodeRef<leptos::html::Textarea> = NodeRef::new();
    Effect::watch(
        move || submit.value().get(),
        move |comment, _, _| {
            if let Some(Ok(_)) = comment.clone() {
                let comment_clone = comment.clone();

                leptos::logging::log!("comment created: {:?}", comment_clone);
                submit.clear();
                input_element.get().map(|input| input.set_value(""));
                on_submit();
            }
        },
        false,
    );
    view! {
        <ActionForm action=submit>
            <label>
                <span>Text</span>
                <textarea name="comment[text]" node_ref=input_element></textarea>
            </label>
            <input type="hidden" name="comment[story_id]" value=story_id />
            <input type="hidden" name="comment[parent_id]" value=parent_id />
            <button type="submit">"Add Comment"</button>
        </ActionForm>
    }
}

#[component]
fn CommentDetail(comment: Comment) -> impl IntoView {
    
    view! {
        <li>
            <p>{comment.text}</p>
            <p class="meta".to_string()>
                <span>by <UserLink user_name=comment.author_name /></span>
                <RelativeTime from=comment.created_at />
            </p>
        </li>
    }
}
