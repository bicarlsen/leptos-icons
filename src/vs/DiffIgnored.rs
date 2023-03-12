#[cfg(feature = "VsDiffIgnored")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDiffIgnored")]
/// *This icon requires the feature* `VsDiffIgnored` *to be enabled*.
#[component]
pub fn DiffIgnored(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.5 1h13l.5.5v13l-.5.5h-13l-.5-.5v-13l.5-.5zM2 14h12V2H2v12zm8-10h2v2l-6 6H4v-2l6-6z" /></svg>
   }
}