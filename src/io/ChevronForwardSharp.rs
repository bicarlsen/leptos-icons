#[cfg(feature = "IoChevronForwardSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronForwardSharp")]
/// *This icon requires the feature* `IoChevronForwardSharp` *to be enabled*.
#[component]
pub fn ChevronForwardSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="184 112 328 256 184 400" style="fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:48px" /></svg>
   }
}