#[cfg(feature = "FaSolidEraser")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidEraser")]
/// *This icon requires the feature* `FaSolidEraser` *to be enabled*.
#[component]
pub fn Eraser(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M258.7 57.4L25.4 290.7c-25 25-25 65.5 0 90.5l80 80c12 12 28.3 18.7 45.3 18.7H256h9.4H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H355.9L486.6 285.3c25-25 25-65.5 0-90.5L349.3 57.4c-25-25-65.5-25-90.5 0zM265.4 416H256l-105.4 0-80-80L195.3 211.3 332.7 348.7 265.4 416z" /></svg>
   }
}