#[cfg(feature = "BiSolidChevronUpSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChevronUpSquare")]
/// *This icon requires the feature* `BiSolidChevronUpSquare` *to be enabled*.
#[component]
pub fn ChevronUpSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 21h14c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2zm7-13.414 5.707 5.707-1.414 1.414L12 10.414l-4.293 4.293-1.414-1.414L12 7.586z" /></svg>
   }
}