#[cfg(feature = "BiRegularTrashAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularTrashAlt")]
/// *This icon requires the feature* `BiRegularTrashAlt` *to be enabled*.
#[component]
pub fn TrashAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M15 2H9c-1.103 0-2 .897-2 2v2H3v2h2v12c0 1.103.897 2 2 2h10c1.103 0 2-.897 2-2V8h2V6h-4V4c0-1.103-.897-2-2-2zM9 4h6v2H9V4zm8 16H7V8h10v12z" /></svg>
   }
}