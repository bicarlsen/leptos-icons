#[cfg(feature = "IoPlaySkipBackOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPlaySkipBackOutline")]
/// *This icon requires the feature* `IoPlaySkipBackOutline` *to be enabled*.
#[component]
pub fn PlaySkipBackOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M400,111V401c0,17.44-17,28.52-31,20.16L121.09,272.79c-12.12-7.25-12.12-26.33,0-33.58L369,90.84C383,82.48,400,93.56,400,111Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><line x1="112" y1="80" x2="112" y2="432" style="fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}