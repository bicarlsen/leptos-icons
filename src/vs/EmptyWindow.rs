#[cfg(feature = "VsEmptyWindow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsEmptyWindow")]
/// *This icon requires the feature* `VsEmptyWindow` *to be enabled*.
#[component]
pub fn EmptyWindow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M4 4h3v1H4v3H3V5H0V4h3V1h1v3zM1 14.5V9h1v5h12V7H8V6h6V4H8V3h6.5l.5.5v11l-.5.5h-13l-.5-.5z" /></svg>
   }
}