#[cfg(feature = "OcSmMoveToTop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmMoveToTop")]
/// *This icon requires the feature* `OcSmMoveToTop` *to be enabled*.
#[component]
pub fn MoveToTop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M8.53 1.22a.749.749 0 0 0-1.06 0L3.72 4.97a.749.749 0 1 0 1.06 1.06l2.47-2.469v6.689a.75.75 0 0 0 1.5 0V3.561l2.47 2.469a.749.749 0 1 0 1.06-1.06L8.53 1.22ZM3.75 13a.75.75 0 0 0 0 1.5h8.5a.75.75 0 0 0 0-1.5h-8.5Z" /></svg>
   }
}