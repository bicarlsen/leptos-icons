#[cfg(feature = "BiSolidMobile")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMobile")]
/// *This icon requires the feature* `BiSolidMobile` *to be enabled*.
#[component]
pub fn Mobile(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 22c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2H8c-1.103 0-2 .897-2 2v16c0 1.103.897 2 2 2h10zm-5-5a1 1 0 1 1 0 2 1 1 0 1 1 0-2z" /></svg>
   }
}