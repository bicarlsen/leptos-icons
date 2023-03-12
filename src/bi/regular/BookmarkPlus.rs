#[cfg(feature = "BiRegularBookmarkPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBookmarkPlus")]
/// *This icon requires the feature* `BiRegularBookmarkPlus` *to be enabled*.
#[component]
pub fn BookmarkPlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13 14v-3h3V9h-3V6h-2v3H8v2h3v3z" /><path d="M20 22V4c0-1.103-.897-2-2-2H6c-1.103 0-2 .897-2 2v18l8-4.572L20 22zM6 10V4h12v14.553l-6-3.428-6 3.428V10z" /></svg>
   }
}