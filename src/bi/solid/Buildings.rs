#[cfg(feature = "BiSolidBuildings")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBuildings")]
/// *This icon requires the feature* `BiSolidBuildings` *to be enabled*.
#[component]
pub fn Buildings(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7 14.001h2v2H7z" /><path d="M19 2h-8a2 2 0 0 0-2 2v6H5c-1.103 0-2 .897-2 2v9a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1V4a2 2 0 0 0-2-2zM5 20v-8h6v8H5zm9-12h-2V6h2v2zm4 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V6h2v2z" /></svg>
   }
}