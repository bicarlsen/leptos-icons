#[cfg(feature = "OcLgTriangleRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgTriangleRight")]
/// *This icon requires the feature* `OcLgTriangleRight` *to be enabled*.
#[component]
pub fn TriangleRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m15.146 12.354-5.792 5.792a.5.5 0 0 1-.854-.353V6.207a.5.5 0 0 1 .854-.353l5.792 5.792a.5.5 0 0 1 0 .708Z" /></svg>
   }
}