#[cfg(feature = "BiSolidMessageAltCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageAltCheck")]
/// *This icon requires the feature* `BiSolidMessageAltCheck` *to be enabled*.
#[component]
pub fn MessageAltCheck(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 4v12c0 1.103.897 2 2 2h3.5l3.5 4 3.5-4H19c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2zm5.707 4.293L11 10.586l4.793-4.793 1.414 1.414L11 13.414 7.293 9.707l1.414-1.414z" /></svg>
   }
}