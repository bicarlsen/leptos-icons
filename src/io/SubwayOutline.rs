#[cfg(feature = "IoSubwayOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSubwayOutline")]
/// *This icon requires the feature* `IoSubwayOutline` *to be enabled*.
#[component]
pub fn SubwayOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="112" y="32" width="288" height="352" rx="48" ry="48" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><line x1="208" y1="80" x2="304" y2="80" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><rect x="112" y="128" width="288" height="96" rx="32" ry="32" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="176" cy="320" r="16" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><circle cx="336" cy="320" r="16" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><line x1="144" y1="464" x2="368" y2="464" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="336" y1="432" x2="384" y2="480" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="176" y1="432" x2="128" y2="480" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}