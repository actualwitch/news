use chrono::{DateTime, FixedOffset, Local};
use chrono_humanize::HumanTime;
use leptos::prelude::*;
use leptos_use::{use_document_visibility, use_interval_fn};
use web_sys::VisibilityState;



pub fn provide_now() {
  let (now, set_now) = signal(Local::now());
  let visibility = use_document_visibility();

  provide_context(now);
  use_interval_fn(
      move || {
          if visibility.get() == VisibilityState::Visible {
              set_now(Local::now());
          }
      },
      30_000,
  );
}



#[component]
pub fn RelativeTime(from: DateTime<FixedOffset>) -> impl IntoView {
    let now = use_context::<ReadSignal<DateTime<Local>>>().expect("now should be provided");
    let human_time = move || {
        let duration = from.signed_duration_since(now.get());
        HumanTime::from(duration).to_string()
    };
    view! { <span title=from.to_string()>{human_time}</span> }
}