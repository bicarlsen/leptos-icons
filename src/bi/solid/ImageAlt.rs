#[cfg(feature = "BiSolidImageAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidImageAlt")]
/// *This icon requires the feature* `BiSolidImageAlt` *to be enabled*.
#[component]
pub fn ImageAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 21h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2zm3-7 2.363 2.363L14 11l5 7H5l3-4z" /></svg>
   }
}