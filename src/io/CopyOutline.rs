#[cfg(feature = "IoCopyOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCopyOutline")]
/// *This icon requires the feature* `IoCopyOutline` *to be enabled*.
#[component]
pub fn CopyOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="128" y="128" width="336" height="336" rx="57" ry="57" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><path d="M383.5,128l.5-24a56.16,56.16,0,0,0-56-56H112a64.19,64.19,0,0,0-64,64V328a56.16,56.16,0,0,0,56,56h24" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}