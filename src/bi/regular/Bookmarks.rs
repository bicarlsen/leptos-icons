#[cfg(feature = "BiRegularBookmarks")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBookmarks")]
/// *This icon requires the feature* `BiRegularBookmarks` *to be enabled*.
#[component]
pub fn Bookmarks(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M14 5H6c-1.103 0-2 .897-2 2v16l6-3.601L16 23V7c0-1.103-.897-2-2-2zm0 14.467-4-2.399-4 2.399V7h8v12.467z" /><path d="M18 1h-8c-1.103 0-2 .897-2 2h8c1.103 0 2 .897 2 2v10.443l2 2.489V3c0-1.103-.897-2-2-2z" /></svg>
   }
}