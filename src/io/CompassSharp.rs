#[cfg(feature = "IoCompassSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCompassSharp")]
/// *This icon requires the feature* `IoCompassSharp` *to be enabled*.
#[component]
pub fn CompassSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="256" r="24" /><path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm48,256L144,368l64-160,160-64Z" /></svg>
   }
}