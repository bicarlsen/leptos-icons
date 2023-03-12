#[cfg(feature = "OcSmIterations")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmIterations")]
/// *This icon requires the feature* `OcSmIterations` *to be enabled*.
#[component]
pub fn Iterations(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M2.5 7.25a4.75 4.75 0 0 1 9.5 0 .75.75 0 0 0 1.5 0 6.25 6.25 0 1 0-6.25 6.25H12v2.146c0 .223.27.335.427.177l2.896-2.896a.25.25 0 0 0 0-.354l-2.896-2.896a.25.25 0 0 0-.427.177V12H7.25A4.75 4.75 0 0 1 2.5 7.25Z" /></svg>
   }
}