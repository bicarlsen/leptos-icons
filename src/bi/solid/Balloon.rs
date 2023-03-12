#[cfg(feature = "BiSolidBalloon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBalloon")]
/// *This icon requires the feature* `BiSolidBalloon` *to be enabled*.
#[component]
pub fn Balloon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11 19.91 10 22h4l-1-2.09c4-.65 7-5.28 7-9.91a8 8 0 0 0-16 0c0 4.63 3.08 9.26 7 9.91zm1-15.66v1.5A4.26 4.26 0 0 0 7.75 10h-1.5A5.76 5.76 0 0 1 12 4.25z" /></svg>
   }
}