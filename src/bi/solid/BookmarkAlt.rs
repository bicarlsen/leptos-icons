#[cfg(feature = "BiSolidBookmarkAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBookmarkAlt")]
/// *This icon requires the feature* `BiSolidBookmarkAlt` *to be enabled*.
#[component]
pub fn BookmarkAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18.5 2h-12C4.57 2 3 3.57 3 5.5V22l7-3.5 7 3.5v-9h5V5.5C22 3.57 20.43 2 18.5 2zm1.5 9h-3V5.5c0-.827.673-1.5 1.5-1.5s1.5.673 1.5 1.5V11z" /></svg>
   }
}