#[cfg(feature = "IoArrowForwardOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoArrowForwardOutline")]
/// *This icon requires the feature* `IoArrowForwardOutline` *to be enabled*.
#[component]
pub fn ArrowForwardOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="268 112 412 256 268 400" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px" /><line x1="392" y1="256" x2="100" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px" /></svg>
   }
}