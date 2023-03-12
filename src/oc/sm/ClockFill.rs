#[cfg(feature = "OcSmClockFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmClockFill")]
/// *This icon requires the feature* `OcSmClockFill` *to be enabled*.
#[component]
pub fn ClockFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8.575-3.25a.825.825 0 1 0-1.65 0v3.5c0 .337.205.64.519.766l2.5 1a.825.825 0 0 0 .612-1.532l-1.981-.793Z" /></svg>
   }
}