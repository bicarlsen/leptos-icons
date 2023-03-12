#[cfg(feature = "RiDeviceFillBatteryShare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillBatteryShare")]
/// *This icon requires the feature* `RiDeviceFillBatteryShare` *to be enabled*.
#[component]
pub fn BatteryShare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M14 2a1 1 0 0 1 1 1v1h3a1 1 0 0 1 1 1v6.2L15 8v3h-1c-2.142 0-4 1.79-4 4v3h2v-3c0-1.05.95-2 2-2h1v3l4-3.2V21a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h3V3a1 1 0 0 1 1-1h4z" /></g></svg>
   }
}