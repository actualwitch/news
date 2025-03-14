use std::cmp::min;

use crate::{
    api::*, constants::{LAMBDA, LAMBDA_FUNCTION, LOADING, NEW, PROFILE, STORY, TITLE_ERROR}, features::{chrono::{provide_now, RelativeTime}, ui::markdown::*, utils::pluralize}, model::{Comment, Story, StoryGetArgs, StoryListItem}
};
use chrono::{DateTime, FixedOffset, Local};
use chrono_humanize::HumanTime;
use comrak::{
    format_html, format_html_with_plugins, markdown_to_html_with_plugins, nodes::NodeValue,
    parse_document, plugins::syntect::SyntectAdapterBuilder, Arena, ExtensionOptions, Options,
    Plugins,
};
use leptos::{either::Either, logging::log, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::{use_navigate, use_params},
    ParamSegment, SsrMode, StaticSegment,
};
use leptos_use::{use_document_visibility, use_interval_fn};
use url::Url;
use web_sys::VisibilityState;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    // 👻
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
    provide_now();

    view! {
        <Stylesheet id="leptos" href="/pkg/news.css" />
        <Router>
            <header>
                <ul>
                    <li>
                        <span class="lambda-home".to_string()>
                            <A href="/">
                                <span class="lambda-icon".to_string()>{LAMBDA}</span>
                                <span class="lambda-text".to_string()>{LAMBDA_FUNCTION}</span>
                            </A>
                        </span>
                    </li>
                    <li class="spacer".to_string()>
                        <span></span>
                    </li>
                    <li>
                        <A href=format!("/{STORY}/{NEW}",)>Bind</A>
                    </li>
                </ul>
            </header>
            <Routes fallback=NotFound>
                <Route path=StaticSegment("") view=StoryList />
                <Route path=(StaticSegment(STORY), StaticSegment(NEW)) view=StoryCreate />
                <Route
                    path=(StaticSegment(STORY), ParamSegment("id"))
                    view=StoryDetail
                    ssr=SsrMode::Async
                />
            </Routes>
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
        <Title text=format!("{} :: {}", "Root Binding", LAMBDA_FUNCTION) />
        <Transition fallback=|| view! { <p>{LOADING}</p> }>
            <ol class="binding".to_string() start=0>
                <For each=stories key=|story| story.id let:story>
                    <li>
                        <div>
                            <StoryLink story_id=story.id title=story.title url=story.url />
                        </div>
                        <div class="meta".to_string()>
                            <A href=format!(
                                "/{}/{}",
                                STORY,
                                story.id,
                            )>{story.comment_count}" "{pluralize(story.comment_count.unwrap_or_default(), "effect", "effects")}</A>
                            <span>owned by <UserLink user_name=story.author_name /></span>
                            <RelativeTime from=story.created_at />
                        </div>
                    </li>
                </For>
            </ol>
        </Transition>
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
        <Transition fallback=|| view! { <p>{LOADING}</p> }>
            {move || {
                Suspend::new(async move {
                    let story = story_res.await;
                    comments.await;
                    view! {
                        {match story {
                            Ok(Story { title, text, id, url, .. }) => {
                                Either::Left(
                                    view! {
                                        <Title text=format!("{} :: {}", title, LAMBDA_FUNCTION) />
                                        <main>
                                            <h4>
                                                <StoryLink story_id=id title=title url=url />
                                            </h4>
                                            <Markdown text=text.unwrap_or_default() />
                                        </main>
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
                        <ol class="effects".to_string()>
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
        </Transition>
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
                navigate(format!("/{STORY}/{}", story.id).as_str(), Default::default());
            }
            _ => {}
        },
        false,
    );

    let success = move || match value.get() {
        Some(Ok(Story { id, .. })) => Some(view! {
            <p class="success">
                <A href=format!("/{STORY}/{id}")>"Story created."</A>
            </p>
        }),
        _ => None,
    };

    let error = move || match value.get() {
        Some(Err(e)) => Some(view! {
            <article class="error">
                <h4>
                    {TITLE_ERROR}
                </h4>
                <p>
                    {match e {
                        ServerFnError::ServerError(e) => e.to_string(),
                        _ => "An error occurred.".to_string(),
                    }}
                </p>
            </article>
        }),
        _ => None,
    };

    view! {
        <Title text=format!("{} :: {}", "New", LAMBDA_FUNCTION) />
        <main>
            <ActionForm action=submit>
                {success} {error} <h1>Bind New Value</h1> <label>
                    <span>Title</span>
                    <input type="text" name="story[title]" />
                </label> <label>
                    <span>Text</span>
                    <textarea name="story[text]"></textarea>
                </label> <label>
                    <span>URL</span>
                    <input type="text" name="story[url]" />
                </label> <button type="submit">"Apply"</button>
            </ActionForm>
        </main>
    }
}

#[component]
fn StoryLink(story_id: i32, title: String, url: Option<String>) -> impl IntoView {
    let story_url = format!("/{STORY}/{story_id}",);

    let domain = url
        .as_ref()
        .and_then(|url| Url::parse(url).ok())
        .and_then(|parsed_url| parsed_url.domain().map(String::from));

    match domain {
        Some(domain) => Either::Left(view! {
            <A href=url.unwrap()>{title}</A>
            <span>" → "</span>
            <A href=format!("/by-domain/{}", domain)>{domain}</A>
        }),
        None => Either::Right(view! { <A href=story_url.clone()>{title}</A> }),
    }
}

#[component]
fn UserLink(user_name: String) -> impl IntoView {
    let profile_url = format!("/{PROFILE}/{user_name}");
    view! { <A href=profile_url>{user_name}</A> }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text=format!("{} :: {}", "Not found", LAMBDA_FUNCTION) />
        <main class="error".to_string()>
            <header>404</header>
            <p>The requested function is undefined</p>
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
    create_effect(move |_| {
        if submit.value().with(|res| res.as_ref().is_some_and(|res| res.is_ok())) {
            submit.clear();
            if let Some(input) = input_element.get() {
                input.set_value("");
            }
            on_submit();
        }
    });
    view! {
        <ActionForm action=submit>
            <label>
                <span>Text</span>
                <textarea name="comment[text]" placeholder="Compose yourself" node_ref=input_element></textarea>
            </label>
            <input type="hidden" name="comment[story_id]" value=story_id />
            <input type="hidden" name="comment[parent_id]" value=parent_id />
            <button type="submit">Apply</button>
        </ActionForm>
    }
}

#[component]
fn CommentDetail(comment: Comment) -> impl IntoView {
    view! {
        <li>
            <div>{comment.text}</div>
            <div class="meta".to_string()>
                <span>by <UserLink user_name=comment.author_name /></span>
                <RelativeTime from=comment.created_at />
            </div>
        </li>
    }
}
