#[cfg(feature = "FaSolidVenus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidVenus")]
/// *This icon requires the feature* `FaSolidVenus` *to be enabled*.
#[component]
pub fn Venus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M64 176a112 112 0 1 1 224 0A112 112 0 1 1 64 176zM208 349.1c81.9-15 144-86.8 144-173.1C352 78.8 273.2 0 176 0S0 78.8 0 176c0 86.3 62.1 158.1 144 173.1V384H112c-17.7 0-32 14.3-32 32s14.3 32 32 32h32v32c0 17.7 14.3 32 32 32s32-14.3 32-32V448h32c17.7 0 32-14.3 32-32s-14.3-32-32-32H208V349.1z" /></svg>
   }
}