#[cfg(feature = "FiMaximize2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiMaximize2")]
/// *This icon requires the feature* `FiMaximize2` *to be enabled*.
#[component]
pub fn Maximize2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="15 3 21 3 21 9" /><polyline points="9 21 3 21 3 15" /><line x1="21" y1="3" x2="14" y2="10" /><line x1="3" y1="21" x2="10" y2="14" /></svg>
   }
}