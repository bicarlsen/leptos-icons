#[cfg(feature = "FaSolidToggleOn")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidToggleOn")]
/// *This icon requires the feature* `FaSolidToggleOn` *to be enabled*.
#[component]
pub fn ToggleOn(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M192 64C86 64 0 150 0 256S86 448 192 448H384c106 0 192-86 192-192s-86-192-192-192H192zm192 96a96 96 0 1 1 0 192 96 96 0 1 1 0-192z" /></svg>
   }
}