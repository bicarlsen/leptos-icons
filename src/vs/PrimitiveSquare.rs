#[cfg(feature = "VsPrimitiveSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsPrimitiveSquare")]
/// *This icon requires the feature* `VsPrimitiveSquare` *to be enabled*.
#[component]
pub fn PrimitiveSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M3.5 4l.5-.5h8l.5.5v8l-.5.5H4l-.5-.5V4zm1 .5v7h7v-7h-7z" /></svg>
   }
}