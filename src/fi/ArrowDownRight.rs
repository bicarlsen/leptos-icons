#[cfg(feature = "FiArrowDownRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiArrowDownRight")]
/// *This icon requires the feature* `FiArrowDownRight` *to be enabled*.
#[component]
pub fn ArrowDownRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="7" y1="7" x2="17" y2="17" /><polyline points="17 7 17 17 7 17" /></svg>
   }
}