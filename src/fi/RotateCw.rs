#[cfg(feature = "FiRotateCw")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiRotateCw")]
/// *This icon requires the feature* `FiRotateCw` *to be enabled*.
#[component]
pub fn RotateCw(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="23 4 23 10 17 10" /><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" /></svg>
   }
}