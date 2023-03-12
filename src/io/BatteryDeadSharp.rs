#[cfg(feature = "IoBatteryDeadSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBatteryDeadSharp")]
/// *This icon requires the feature* `IoBatteryDeadSharp` *to be enabled*.
#[component]
pub fn BatteryDeadSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="32" y="144" width="400" height="224" style="fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /><line x1="480" y1="218.67" x2="480" y2="293.33" style="fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}