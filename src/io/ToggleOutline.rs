#[cfg(feature = "IoToggleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoToggleOutline")]
/// *This icon requires the feature* `IoToggleOutline` *to be enabled*.
#[component]
pub fn ToggleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="368" cy="256" r="128" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><rect x="16" y="128" width="480" height="256" rx="128" ry="128" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}