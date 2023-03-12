#[cfg(feature = "VsDeviceMobile")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDeviceMobile")]
/// *This icon requires the feature* `VsDeviceMobile` *to be enabled*.
#[component]
pub fn DeviceMobile(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M4.5 1h8l.5.5v13l-.5.5h-8l-.5-.5v-13l.5-.5zM5 14h7V2H5v12zm2.5-2h2v1h-2v-1z" /></svg>
   }
}