#[cfg(feature = "OcLgTriangleUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgTriangleUp")]
/// *This icon requires the feature* `OcLgTriangleUp` *to be enabled*.
#[component]
pub fn TriangleUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m12.354 8.854 5.792 5.792a.5.5 0 0 1-.353.854H6.207a.5.5 0 0 1-.353-.854l5.792-5.792a.5.5 0 0 1 .708 0Z" /></svg>
   }
}