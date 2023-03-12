#[cfg(feature = "FiCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCircle")]
/// *This icon requires the feature* `FiCircle` *to be enabled*.
#[component]
pub fn Circle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /></svg>
   }
}