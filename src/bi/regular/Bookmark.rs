#[cfg(feature = "BiRegularBookmark")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBookmark")]
/// *This icon requires the feature* `BiRegularBookmark` *to be enabled*.
#[component]
pub fn Bookmark(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 2H6c-1.103 0-2 .897-2 2v18l8-4.572L20 22V4c0-1.103-.897-2-2-2zm0 16.553-6-3.428-6 3.428V4h12v14.553z" /></svg>
   }
}