#[cfg(feature = "RiDeviceFillCpu")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillCpu")]
/// *This icon requires the feature* `RiDeviceFillCpu` *to be enabled*.
#[component]
pub fn Cpu(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M14 20h-4v2H8v-2H5a1 1 0 0 1-1-1v-3H2v-2h2v-4H2V8h2V5a1 1 0 0 1 1-1h3V2h2v2h4V2h2v2h3a1 1 0 0 1 1 1v3h2v2h-2v4h2v2h-2v3a1 1 0 0 1-1 1h-3v2h-2v-2zM7 7v4h4V7H7z" /></g></svg>
   }
}