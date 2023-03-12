#[cfg(feature = "IoBluetoothOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBluetoothOutline")]
/// *This icon requires the feature* `IoBluetoothOutline` *to be enabled*.
#[component]
pub fn BluetoothOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="144 352 368 160 256 48 256 464 368 352 144 160" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}