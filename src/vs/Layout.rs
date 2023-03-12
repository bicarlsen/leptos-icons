#[cfg(feature = "VsLayout")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLayout")]
/// *This icon requires the feature* `VsLayout` *to be enabled*.
#[component]
pub fn Layout(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M3 2L2 3V13L3 14H7L8 13V3L7 2H3ZM3 13V3H7V13H3Z" /><path d="M10 3L11 2H14L15 3V6L14 7H11L10 6V3ZM11 3V6H14V3H11Z" /><path d="M10 10L11 9H14L15 10V13L14 14H11L10 13V10ZM11 10V13H14V10H11Z" /></svg>
   }
}