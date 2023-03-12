#[cfg(feature = "FaSolidComputerMouse")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidComputerMouse")]
/// *This icon requires the feature* `FaSolidComputerMouse` *to be enabled*.
#[component]
pub fn ComputerMouse(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M0 192H176V0H160C71.6 0 0 71.6 0 160v32zm0 32V352c0 88.4 71.6 160 160 160h64c88.4 0 160-71.6 160-160V224H192 0zm384-32V160C384 71.6 312.4 0 224 0H208V192H384z" /></svg>
   }
}