#[cfg(feature = "FaSolidU")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidU")]
/// *This icon requires the feature* `FaSolidU` *to be enabled*.
#[component]
pub fn U(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M32 32c17.7 0 32 14.3 32 32V288c0 70.7 57.3 128 128 128s128-57.3 128-128V64c0-17.7 14.3-32 32-32s32 14.3 32 32V288c0 106-86 192-192 192S0 394 0 288V64C0 46.3 14.3 32 32 32z" /></svg>
   }
}