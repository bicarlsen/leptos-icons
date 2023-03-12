#[cfg(feature = "RiDeviceFillBarcode")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillBarcode")]
/// *This icon requires the feature* `RiDeviceFillBarcode` *to be enabled*.
#[component]
pub fn Barcode(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2 4h2v16H2V4zm4 0h2v16H6V4zm3 0h3v16H9V4zm4 0h2v16h-2V4zm3 0h2v16h-2V4zm3 0h3v16h-3V4z" /></g></svg>
   }
}