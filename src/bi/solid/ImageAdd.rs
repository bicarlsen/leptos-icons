#[cfg(feature = "BiSolidImageAdd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidImageAdd")]
/// *This icon requires the feature* `BiSolidImageAdd` *to be enabled*.
#[component]
pub fn ImageAdd(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m9 13 3-4 3 4.5V12h4V5c0-1.103-.897-2-2-2H4c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h8v-4H5l3-4 1 2z" /><path d="M19 14h-2v3h-3v2h3v3h2v-3h3v-2h-3z" /></svg>
   }
}