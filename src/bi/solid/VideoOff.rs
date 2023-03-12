#[cfg(feature = "BiSolidVideoOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidVideoOff")]
/// *This icon requires the feature* `BiSolidVideoOff` *to be enabled*.
#[component]
pub fn VideoOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 19h10.879L2.145 6.265A1.977 1.977 0 0 0 2 7v10c0 1.103.897 2 2 2zM18 7c0-1.103-.897-2-2-2H6.414L3.707 2.293 2.293 3.707l18 18 1.414-1.414L18 16.586v-2.919L22 17V7l-4 3.333V7z" /></svg>
   }
}