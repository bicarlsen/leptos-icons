#[cfg(feature = "FiTrendingUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiTrendingUp")]
/// *This icon requires the feature* `FiTrendingUp` *to be enabled*.
#[component]
pub fn TrendingUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="23 6 13.5 15.5 8.5 10.5 1 18" /><polyline points="17 6 23 6 23 12" /></svg>
   }
}