#[cfg(feature = "BiSolidCommentError")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCommentError")]
/// *This icon requires the feature* `BiSolidCommentError` *to be enabled*.
#[component]
pub fn CommentError(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .897-2 2v18l4-4h14c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm-7 13h-2v-2h2v2zm0-4h-2V5h2v6z" /></svg>
   }
}