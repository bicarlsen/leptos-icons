#[cfg(feature = "RiDeviceLineBarcodeBox")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceLineBarcodeBox")]
/// *This icon requires the feature* `RiDeviceLineBarcodeBox` *to be enabled*.
#[component]
pub fn BarcodeBox(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M4 5v14h16V5H4zM3 3h18a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm3 4h3v10H6V7zm4 0h2v10h-2V7zm3 0h1v10h-1V7zm2 0h3v10h-3V7z" /></g></svg>
   }
}