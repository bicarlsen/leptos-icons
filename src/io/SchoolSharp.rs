#[cfg(feature = "IoSchoolSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSchoolSharp")]
/// *This icon requires the feature* `IoSchoolSharp` *to be enabled*.
#[component]
pub fn SchoolSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="256 370.43 96 279 96 377.42 256 466.3 416 377.42 416 279 256 370.43" /><polygon points="512.25 192 256 45.57 -0.25 192 256 338.43 464 219.57 464 384 512 384 512 192.14 512.25 192" /></svg>
   }
}