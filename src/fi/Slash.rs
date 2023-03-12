#[cfg(feature = "FiSlash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiSlash")]
/// *This icon requires the feature* `FiSlash` *to be enabled*.
#[component]
pub fn Slash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><line x1="4.93" y1="4.93" x2="19.07" y2="19.07" /></svg>
   }
}