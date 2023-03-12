#[cfg(feature = "RiDeviceLineSmartphone")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceLineSmartphone")]
/// *This icon requires the feature* `RiDeviceLineSmartphone` *to be enabled*.
#[component]
pub fn Smartphone(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M7 4v16h10V4H7zM6 2h12a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1zm6 15a1 1 0 1 1 0 2 1 1 0 0 1 0-2z" /></g></svg>
   }
}