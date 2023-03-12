#[cfg(feature = "OcSmSquareFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmSquareFill")]
/// *This icon requires the feature* `OcSmSquareFill` *to be enabled*.
#[component]
pub fn SquareFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M5.75 4h4.5c.966 0 1.75.784 1.75 1.75v4.5A1.75 1.75 0 0 1 10.25 12h-4.5A1.75 1.75 0 0 1 4 10.25v-4.5C4 4.784 4.784 4 5.75 4Z" /></svg>
   }
}