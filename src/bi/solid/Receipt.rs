#[cfg(feature = "BiSolidReceipt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidReceipt")]
/// *This icon requires the feature* `BiSolidReceipt` *to be enabled*.
#[component]
pub fn Receipt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 12v6a1 1 0 0 1-2 0V4a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v14c0 1.654 1.346 3 3 3h14c1.654 0 3-1.346 3-3v-6h-2zm-6-1v2H6v-2h8zM6 9V7h8v2H6zm8 6v2h-3v-2h3z" /></svg>
   }
}