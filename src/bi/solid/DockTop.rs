#[cfg(feature = "BiSolidDockTop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDockTop")]
/// *This icon requires the feature* `BiSolidDockTop` *to be enabled*.
#[component]
pub fn DockTop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 5v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2zm2 14v-9h14.001l.001 9H5z" /></svg>
   }
}