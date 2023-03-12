#[cfg(feature = "CgPolaroid")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPolaroid")]
/// *This icon requires the feature* `CgPolaroid` *to be enabled*.
#[component]
pub fn Polaroid(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M3 4C3 2.89543 3.89543 2 5 2H19C20.1046 2 21 2.89543 21 4V20C21 21.1046 20.1046 22 19 22H5C3.89543 22 3 21.1046 3 20V4ZM5 4H19V15H5V4Z" fill="currentColor" /></svg>
   }
}