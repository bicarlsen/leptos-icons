#[cfg(feature = "BiSolidChevronRightSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChevronRightSquare")]
/// *This icon requires the feature* `BiSolidChevronRightSquare` *to be enabled*.
#[component]
pub fn ChevronRightSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 5v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2zm6.293 2.707 1.414-1.414L16.414 12l-5.707 5.707-1.414-1.414L13.586 12 9.293 7.707z" /></svg>
   }
}