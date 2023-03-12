#[cfg(feature = "BiSolidMessageAltError")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageAltError")]
/// *This icon requires the feature* `BiSolidMessageAltError` *to be enabled*.
#[component]
pub fn MessageAltError(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 4v12c0 1.103.897 2 2 2h3.5l3.5 4 3.5-4H19c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2zm8 1h2v6h-2V5zm0 8h2v2h-2v-2z" /></svg>
   }
}