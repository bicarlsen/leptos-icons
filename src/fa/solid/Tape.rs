#[cfg(feature = "FaSolidTape")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidTape")]
/// *This icon requires the feature* `FaSolidTape` *to be enabled*.
#[component]
pub fn Tape(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M380.8 416c41.5-40.7 67.2-97.3 67.2-160C448 132.3 347.7 32 224 32S0 132.3 0 256S100.3 480 224 480H544c17.7 0 32-14.3 32-32s-14.3-32-32-32H380.8zM224 160a96 96 0 1 1 0 192 96 96 0 1 1 0-192zm64 96a64 64 0 1 0 -128 0 64 64 0 1 0 128 0z" /></svg>
   }
}