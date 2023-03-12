#[cfg(feature = "AiTwotoneBuild")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiTwotoneBuild")]
/// *This icon requires the feature* `AiTwotoneBuild` *to be enabled*.
#[component]
pub fn Build(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024"><path fill="#D9D9D9" d="M144 546h200v200H144zm268-268h200v200H412z" /><path d="M916 210H376c-17.7 0-32 14.3-32 32v236H108c-17.7 0-32 14.3-32 32v272c0 17.7 14.3 32 32 32h540c17.7 0 32-14.3 32-32V546h236c17.7 0 32-14.3 32-32V242c0-17.7-14.3-32-32-32zM344 746H144V546h200v200zm268 0H412V546h200v200zm0-268H412V278h200v200zm268 0H680V278h200v200z" /></svg>
   }
}