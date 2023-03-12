#[cfg(feature = "BiSolidPolygon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPolygon")]
/// *This icon requires the feature* `BiSolidPolygon` *to be enabled*.
#[component]
pub fn Polygon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m21.707 7.293-5-5A.996.996 0 0 0 16 2H8a.996.996 0 0 0-.707.293l-5 5A.996.996 0 0 0 2 8v8c0 .266.105.52.293.707l5 5A.996.996 0 0 0 8 22h8c.266 0 .52-.105.707-.293l5-5A.996.996 0 0 0 22 16V8a.996.996 0 0 0-.293-.707z" /></svg>
   }
}