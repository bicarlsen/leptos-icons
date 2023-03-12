#[cfg(feature = "IoScanOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoScanOutline")]
/// *This icon requires the feature* `IoScanOutline` *to be enabled*.
#[component]
pub fn ScanOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M336,448h56a56,56,0,0,0,56-56V336" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M448,176V120a56,56,0,0,0-56-56H336" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M176,448H120a56,56,0,0,1-56-56V336" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M64,176V120a56,56,0,0,1,56-56h56" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}