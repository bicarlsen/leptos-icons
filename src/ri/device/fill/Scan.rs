#[cfg(feature = "RiDeviceFillScan")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillScan")]
/// *This icon requires the feature* `RiDeviceFillScan` *to be enabled*.
#[component]
pub fn Scan(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M4.257 5.671L12 13.414 13.414 12 5.671 4.257A9.959 9.959 0 0 1 12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12c0-2.401.846-4.605 2.257-6.329z" /></g></svg>
   }
}