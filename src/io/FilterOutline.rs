#[cfg(feature = "IoFilterOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFilterOutline")]
/// *This icon requires the feature* `IoFilterOutline` *to be enabled*.
#[component]
pub fn FilterOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><line x1="32" y1="144" x2="480" y2="144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="112" y1="256" x2="400" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="208" y1="368" x2="304" y2="368" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}