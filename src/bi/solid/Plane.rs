#[cfg(feature = "BiSolidPlane")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPlane")]
/// *This icon requires the feature* `BiSolidPlane` *to be enabled*.
#[component]
pub fn Plane(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M22 16.21v-1.895L14 8V4a2 2 0 0 0-4 0v4.105L2 14.42v1.789l8-2.81V18l-3 2v2l5-2 5 2v-2l-3-2v-4.685l8 2.895z" /></svg>
   }
}