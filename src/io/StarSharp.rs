#[cfg(feature = "IoStarSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoStarSharp")]
/// *This icon requires the feature* `IoStarSharp` *to be enabled*.
#[component]
pub fn StarSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M496,203.3H312.36L256,32,199.64,203.3H16L166.21,308.7,107.71,480,256,373.84,404.29,480,345.68,308.7Z" /></svg>
   }
}