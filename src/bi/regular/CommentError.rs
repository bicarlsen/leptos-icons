#[cfg(feature = "BiRegularCommentError")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCommentError")]
/// *This icon requires the feature* `BiRegularCommentError` *to be enabled*.
#[component]
pub fn CommentError(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11 6h2v5h-2zm0 6h2v2h-2z" /><path d="M20 2H4c-1.103 0-2 .897-2 2v18l5.333-4H20c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm0 14H6.667L4 18V4h16v12z" /></svg>
   }
}