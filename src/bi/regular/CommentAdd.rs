#[cfg(feature = "BiRegularCommentAdd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCommentAdd")]
/// *This icon requires the feature* `BiRegularCommentAdd` *to be enabled*.
#[component]
pub fn CommentAdd(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11 14h2v-3h3V9h-3V6h-2v3H8v2h3z" /><path d="M20 2H4c-1.103 0-2 .897-2 2v18l5.333-4H20c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm0 14H6.667L4 18V4h16v12z" /></svg>
   }
}