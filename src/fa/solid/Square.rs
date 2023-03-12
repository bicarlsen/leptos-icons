#[cfg(feature = "FaSolidSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidSquare")]
/// *This icon requires the feature* `FaSolidSquare` *to be enabled*.
#[component]
pub fn Square(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M0 96C0 60.7 28.7 32 64 32H384c35.3 0 64 28.7 64 64V416c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V96z" /></svg>
   }
}