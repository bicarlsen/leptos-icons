#[cfg(feature = "BsPlayCircleFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsPlayCircleFill")]
/// *This icon requires the feature* `BsPlayCircleFill` *to be enabled*.
#[component]
pub fn PlayCircleFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-play-circle-fill" viewBox="0 0 16 16"><path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zM6.79 5.093A.5.5 0 0 0 6 5.5v5a.5.5 0 0 0 .79.407l3.5-2.5a.5.5 0 0 0 0-.814l-3.5-2.5z" /></svg>
   }
}