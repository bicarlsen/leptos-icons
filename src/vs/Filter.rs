#[cfg(feature = "VsFilter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsFilter")]
/// *This icon requires the feature* `VsFilter` *to be enabled*.
#[component]
pub fn Filter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M15 2v1.67l-5 4.759V14H6V8.429l-5-4.76V2h14zM7 8v5h2V8l5-4.76V3H2v.24L7 8z" /></svg>
   }
}