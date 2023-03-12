#[cfg(feature = "BiSolidMessageAltDetail")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageAltDetail")]
/// *This icon requires the feature* `BiSolidMessageAltDetail` *to be enabled*.
#[component]
pub fn MessageAltDetail(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m8.5 18 3.5 4 3.5-4H19c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h3.5zM7 7h10v2H7V7zm0 4h7v2H7v-2z" /></svg>
   }
}