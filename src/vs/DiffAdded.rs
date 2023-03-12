#[cfg(feature = "VsDiffAdded")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDiffAdded")]
/// *This icon requires the feature* `VsDiffAdded` *to be enabled*.
#[component]
pub fn DiffAdded(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.5 1h12l.5.5v12l-.5.5h-12l-.5-.5v-12l.5-.5zM2 13h11V2H2v11z" /><path fill-rule="evenodd" clip-rule="evenodd" d="M8 4H7v3H4v1h3v3h1V8h3V7H8V4z" /></svg>
   }
}