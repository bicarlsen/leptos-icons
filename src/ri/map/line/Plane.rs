#[cfg(feature = "RiMapLinePlane")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMapLinePlane")]
/// *This icon requires the feature* `RiMapLinePlane` *to be enabled*.
#[component]
pub fn Plane(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M14 8.947L22 14v2l-8-2.526v5.36l3 1.666V22l-4.5-1L8 22v-1.5l3-1.667v-5.36L3 16v-2l8-5.053V3.5a1.5 1.5 0 0 1 3 0v5.447z" /></g></svg>
   }
}