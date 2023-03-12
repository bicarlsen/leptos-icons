#[cfg(feature = "IoArrowUndoOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoArrowUndoOutline")]
/// *This icon requires the feature* `IoArrowUndoOutline` *to be enabled*.
#[component]
pub fn ArrowUndoOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M240,424V328c116.4,0,159.39,33.76,208,96,0-119.23-39.57-240-208-240V88L64,256Z" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}