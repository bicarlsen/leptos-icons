#[cfg(feature = "BiSolidBookBookmark")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBookBookmark")]
/// *This icon requires the feature* `BiSolidBookBookmark` *to be enabled*.
#[component]
pub fn BookBookmark(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 2H6c-1.206 0-3 .799-3 3v14c0 2.201 1.794 3 3 3h15v-2H6.012C5.55 19.988 5 19.806 5 19c0-.101.009-.191.024-.273.112-.576.584-.717.988-.727H21V4a2 2 0 0 0-2-2zm0 9-2-1-2 1V4h4v7z" /></svg>
   }
}