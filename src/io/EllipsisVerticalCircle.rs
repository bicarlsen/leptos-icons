#[cfg(feature = "IoEllipsisVerticalCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoEllipsisVerticalCircle")]
/// *This icon requires the feature* `IoEllipsisVerticalCircle` *to be enabled*.
#[component]
pub fn EllipsisVerticalCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="256" r="26" /><circle cx="256" cy="346" r="26" /><circle cx="256" cy="166" r="26" /><path d="M448,256c0-106-86-192-192-192S64,150,64,256s86,192,192,192S448,362,448,256Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}