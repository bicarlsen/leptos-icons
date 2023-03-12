#[cfg(feature = "BiSolidZoomOut")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidZoomOut")]
/// *This icon requires the feature* `BiSolidZoomOut` *to be enabled*.
#[component]
pub fn ZoomOut(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10 18a7.952 7.952 0 0 0 4.897-1.688l4.396 4.396 1.414-1.414-4.396-4.396A7.952 7.952 0 0 0 18 10c0-4.411-3.589-8-8-8s-8 3.589-8 8 3.589 8 8 8zM6 9h8v2H6V9z" /></svg>
   }
}