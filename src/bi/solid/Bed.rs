#[cfg(feature = "BiSolidBed")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBed")]
/// *This icon requires the feature* `BiSolidBed` *to be enabled*.
#[component]
pub fn Bed(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 9.556V3h-2v2H6V3H4v6.557C2.81 10.25 2 11.526 2 13v4a1 1 0 0 0 1 1h1v4h2v-4h12v4h2v-4h1a1 1 0 0 0 1-1v-4c0-1.474-.811-2.75-2-3.444zM11 9H6V7h5v2zm7 0h-5V7h5v2z" /></svg>
   }
}