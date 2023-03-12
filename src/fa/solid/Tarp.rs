#[cfg(feature = "FaSolidTarp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidTarp")]
/// *This icon requires the feature* `FaSolidTarp` *to be enabled*.
#[component]
pub fn Tarp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M576 128c0-35.3-28.7-64-64-64H64C28.7 64 0 92.7 0 128V384c0 35.3 28.7 64 64 64l352 0 0-128c0-17.7 14.3-32 32-32H576V128zM448 448L576 320H448l0 128zM96 128a32 32 0 1 1 0 64 32 32 0 1 1 0-64z" /></svg>
   }
}