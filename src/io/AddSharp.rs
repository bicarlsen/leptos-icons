#[cfg(feature = "IoAddSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoAddSharp")]
/// *This icon requires the feature* `IoAddSharp` *to be enabled*.
#[component]
pub fn AddSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><line x1="256" y1="112" x2="256" y2="400" style="fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px" /><line x1="400" y1="256" x2="112" y2="256" style="fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}