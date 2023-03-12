#[cfg(feature = "IoGitNetworkOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoGitNetworkOutline")]
/// *This icon requires the feature* `IoGitNetworkOutline` *to be enabled*.
#[component]
pub fn GitNetworkOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="128" cy="96" r="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="256" cy="416" r="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="256" y1="256" x2="256" y2="368" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="384" cy="96" r="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M128,144c0,74.67,68.92,112,128,112" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M384,144c0,74.67-68.92,112-128,112" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}