#[cfg(feature = "OcCheckCircleFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcCheckCircleFill")]
/// *This icon requires the feature* `OcCheckCircleFill` *to be enabled*.
#[component]
pub fn CheckCircleFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 12 12"><path d="M6 0a6 6 0 1 1 0 12A6 6 0 0 1 6 0Zm-.705 8.737L9.63 4.403 8.392 3.166 5.295 6.263l-1.7-1.702L2.356 5.8l2.938 2.938Z" /></svg>
   }
}