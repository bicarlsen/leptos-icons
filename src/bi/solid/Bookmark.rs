#[cfg(feature = "BiSolidBookmark")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBookmark")]
/// *This icon requires the feature* `BiSolidBookmark` *to be enabled*.
#[component]
pub fn Bookmark(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 10.132v-6c0-1.103-.897-2-2-2H7c-1.103 0-2 .897-2 2V22l7-4.666L19 22V10.132z" /></svg>
   }
}