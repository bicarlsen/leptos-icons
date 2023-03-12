#[cfg(feature = "OcSmChevronLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmChevronLeft")]
/// *This icon requires the feature* `OcSmChevronLeft` *to be enabled*.
#[component]
pub fn ChevronLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M9.78 12.78a.75.75 0 0 1-1.06 0L4.47 8.53a.75.75 0 0 1 0-1.06l4.25-4.25a.751.751 0 0 1 1.042.018.751.751 0 0 1 .018 1.042L6.06 8l3.72 3.72a.75.75 0 0 1 0 1.06Z" /></svg>
   }
}