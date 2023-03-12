#[cfg(feature = "RiDeviceFillKeyboard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillKeyboard")]
/// *This icon requires the feature* `RiDeviceFillKeyboard` *to be enabled*.
#[component]
pub fn Keyboard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 17h18v2H3v-2zm0-6h3v3H3v-3zm5 0h3v3H8v-3zM3 5h3v3H3V5zm10 0h3v3h-3V5zm5 0h3v3h-3V5zm-5 6h3v3h-3v-3zm5 0h3v3h-3v-3zM8 5h3v3H8V5z" /></g></svg>
   }
}