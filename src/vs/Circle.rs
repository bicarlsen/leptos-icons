#[cfg(feature = "VsCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsCircle")]
/// *This icon requires the feature* `VsCircle` *to be enabled*.
#[component]
pub fn Circle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M8 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8zm2.61-4a2.61 2.61 0 1 1-5.22 0 2.61 2.61 0 0 1 5.22 0zM8 5.246z" /></svg>
   }
}