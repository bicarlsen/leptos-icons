#[cfg(feature = "IoAttachSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoAttachSharp")]
/// *This icon requires the feature* `IoAttachSharp` *to be enabled*.
#[component]
pub fn AttachSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M216.08,192V335.55a40.08,40.08,0,0,0,80.15,0L296.36,147a67.94,67.94,0,1,0-135.87,0V336.82a95.51,95.51,0,0,0,191,0V159.44" style="fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}