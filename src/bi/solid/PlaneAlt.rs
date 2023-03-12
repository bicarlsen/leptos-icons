#[cfg(feature = "BiSolidPlaneAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPlaneAlt")]
/// *This icon requires the feature* `BiSolidPlaneAlt` *to be enabled*.
#[component]
pub fn PlaneAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3.414 13.778 2 15.192l4.949 2.121 2.122 4.95 1.414-1.414-.707-3.536L13.091 14l3.61 7.704 1.339-1.339-1.19-10.123 2.828-2.829a2 2 0 1 0-2.828-2.828l-2.903 2.903L3.824 6.297 2.559 7.563l7.644 3.67-3.253 3.253-3.536-.708z" /></svg>
   }
}