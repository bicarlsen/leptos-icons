#[cfg(feature = "OcSmTriangleDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmTriangleDown")]
/// *This icon requires the feature* `OcSmTriangleDown` *to be enabled*.
#[component]
pub fn TriangleDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="m4.427 7.427 3.396 3.396a.25.25 0 0 0 .354 0l3.396-3.396A.25.25 0 0 0 11.396 7H4.604a.25.25 0 0 0-.177.427Z" /></svg>
   }
}