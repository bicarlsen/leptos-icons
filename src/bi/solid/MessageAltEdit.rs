#[cfg(feature = "BiSolidMessageAltEdit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageAltEdit")]
/// *This icon requires the feature* `BiSolidMessageAltEdit` *to be enabled*.
#[component]
pub fn MessageAltEdit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 2c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h3.5l3.5 4 3.5-4H19c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2H5zm4.302 11.987h-1.8v-1.799l4.978-4.97 1.798 1.799-4.976 4.97zm5.823-5.817-1.798-1.799L14.698 5l1.8 1.799-1.373 1.371z" /></svg>
   }
}