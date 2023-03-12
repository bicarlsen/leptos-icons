#[cfg(feature = "BiSolidBatteryCharging")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBatteryCharging")]
/// *This icon requires the feature* `BiSolidBatteryCharging` *to be enabled*.
#[component]
pub fn BatteryCharging(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 10V8a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-2h2v-4h-2zM9 17l2-3.89L7 12l6-5-1 3.89L16 12l-7 5z" /></svg>
   }
}