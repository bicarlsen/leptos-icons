#[cfg(feature = "BiSolidPaste")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPaste")]
/// *This icon requires the feature* `BiSolidPaste` *to be enabled*.
#[component]
pub fn Paste(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M9 4h6v2H9zm11 7h-7a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h7a2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2z" /><path d="M21 9V6a2 2 0 0 0-2-2h-2a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2H5a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h4v-9a2 2 0 0 1 2-2h10zM9 6V4h6v2H9z" /></svg>
   }
}