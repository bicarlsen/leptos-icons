#[cfg(feature = "IoBookmarksSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBookmarksSharp")]
/// *This icon requires the feature* `IoBookmarksSharp` *to be enabled*.
#[component]
pub fn BookmarksSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="112 0 112 48 416 48 416 416 464 448 464 0 112 0" /><polygon points="48 80 48 512 216 388 384 512 384 80 48 80" /></svg>
   }
}