#[cfg(feature = "BiSolidCommentAdd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCommentAdd")]
/// *This icon requires the feature* `BiSolidCommentAdd` *to be enabled*.
#[component]
pub fn CommentAdd(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .897-2 2v18l4-4h14c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm-3 9h-4v4h-2v-4H7V9h4V5h2v4h4v2z" /></svg>
   }
}