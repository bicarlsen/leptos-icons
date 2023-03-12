#[cfg(feature = "FiCheckCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCheckCircle")]
/// *This icon requires the feature* `FiCheckCircle` *to be enabled*.
#[component]
pub fn CheckCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" /><polyline points="22 4 12 14.01 9 11.01" /></svg>
   }
}