#[cfg(feature = "RiDeviceLineWirelessCharging")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceLineWirelessCharging")]
/// *This icon requires the feature* `RiDeviceLineWirelessCharging` *to be enabled*.
#[component]
pub fn WirelessCharging(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0L24 0 24 24 0 24z" /><path d="M3.929 4.929l1.414 1.414C3.895 7.791 3 9.791 3 12c0 2.21.895 4.21 2.343 5.657L3.93 19.07C2.119 17.261 1 14.761 1 12s1.12-5.261 2.929-7.071zm16.142 0C21.881 6.739 23 9.239 23 12s-1.12 5.262-2.929 7.071l-1.414-1.414C20.105 16.209 21 14.209 21 12s-.895-4.208-2.342-5.656L20.07 4.93zM13 5v6h3l-5 8v-6H8l5-8zM6.757 7.757l1.415 1.415C7.448 9.895 7 10.895 7 12c0 1.105.448 2.105 1.172 2.828l-1.415 1.415C5.672 15.157 5 13.657 5 12c0-1.657.672-3.157 1.757-4.243zm10.487.001C18.329 8.844 19 10.344 19 12c0 1.657-.672 3.157-1.757 4.243l-1.415-1.415C16.552 14.105 17 13.105 17 12c0-1.104-.447-2.104-1.17-2.827l1.414-1.415z" /></g></svg>
   }
}