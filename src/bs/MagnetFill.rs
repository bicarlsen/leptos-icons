#[cfg(feature = "BsMagnetFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsMagnetFill")]
/// *This icon requires the feature* `BsMagnetFill` *to be enabled*.
#[component]
pub fn MagnetFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-magnet-fill" viewBox="0 0 16 16"><path d="M15 12h-4v3h4v-3ZM5 12H1v3h4v-3ZM0 8a8 8 0 1 1 16 0v8h-6V8a2 2 0 1 0-4 0v8H0V8Z" /></svg>
   }
}