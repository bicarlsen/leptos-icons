#[cfg(feature = "OcSmTriangleLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmTriangleLeft")]
/// *This icon requires the feature* `OcSmTriangleLeft` *to be enabled*.
#[component]
pub fn TriangleLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M9.573 4.427 6.177 7.823a.25.25 0 0 0 0 .354l3.396 3.396a.25.25 0 0 0 .427-.177V4.604a.25.25 0 0 0-.427-.177Z" /></svg>
   }
}