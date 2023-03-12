#[cfg(feature = "FaSolidJ")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidJ")]
/// *This icon requires the feature* `FaSolidJ` *to be enabled*.
#[component]
pub fn J(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M288 32c17.7 0 32 14.3 32 32V320c0 88.4-71.6 160-160 160S0 408.4 0 320V288c0-17.7 14.3-32 32-32s32 14.3 32 32v32c0 53 43 96 96 96s96-43 96-96V64c0-17.7 14.3-32 32-32z" /></svg>
   }
}