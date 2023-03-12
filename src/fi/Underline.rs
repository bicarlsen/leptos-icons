#[cfg(feature = "FiUnderline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiUnderline")]
/// *This icon requires the feature* `FiUnderline` *to be enabled*.
#[component]
pub fn Underline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3" /><line x1="4" y1="21" x2="20" y2="21" /></svg>
   }
}