#[cfg(feature = "FiToggleLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiToggleLeft")]
/// *This icon requires the feature* `FiToggleLeft` *to be enabled*.
#[component]
pub fn ToggleLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="1" y="5" width="22" height="14" rx="7" ry="7" /><circle cx="8" cy="12" r="3" /></svg>
   }
}