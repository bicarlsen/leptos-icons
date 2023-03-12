#[cfg(feature = "FiFramer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiFramer")]
/// *This icon requires the feature* `FiFramer` *to be enabled*.
#[component]
pub fn Framer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 16V9h14V2H5l14 14h-7m-7 0l7 7v-7m-7 0h7" /></svg>
   }
}