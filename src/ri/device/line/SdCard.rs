#[cfg(feature = "RiDeviceLineSdCard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceLineSdCard")]
/// *This icon requires the feature* `RiDeviceLineSdCard` *to be enabled*.
#[component]
pub fn SdCard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M6 7.828V20h12V4H9.828L6 7.828zm-1.707-1.12L9 2h10a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7.414a1 1 0 0 1 .293-.707zM15 5h2v4h-2V5zm-3 0h2v4h-2V5zM9 6h2v3H9V6z" /></g></svg>
   }
}