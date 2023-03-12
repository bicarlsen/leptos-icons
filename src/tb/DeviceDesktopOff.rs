#[cfg(feature = "TbDeviceDesktopOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDeviceDesktopOff")]
/// *This icon requires the feature* `TbDeviceDesktopOff` *to be enabled*.
#[component]
pub fn DeviceDesktopOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-desktop-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M8 4h12a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1m-4 0h-12a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1" /><path d="M7 20l10 0" /><path d="M9 16l0 4" /><path d="M15 16l0 4" /><path d="M3 3l18 18" /></svg>
   }
}