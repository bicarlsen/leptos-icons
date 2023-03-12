#[cfg(feature = "VsPlay")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsPlay")]
/// *This icon requires the feature* `VsPlay` *to be enabled*.
#[component]
pub fn Play(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M3.78 2L3 2.41v12l.78.42 9-6V8l-9-6zM4 13.48V3.35l7.6 5.07L4 13.48z" /></svg>
   }
}