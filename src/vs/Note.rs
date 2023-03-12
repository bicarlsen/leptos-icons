#[cfg(feature = "VsNote")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsNote")]
/// *This icon requires the feature* `VsNote` *to be enabled*.
#[component]
pub fn Note(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.5 2h13l.5.5v10l-.5.5h-13l-.5-.5v-10l.5-.5zM2 3v9h12V3H2zm2 2h8v1H4V5zm6 2H4v1h6V7zM4 9h4v1H4V9z" /></svg>
   }
}