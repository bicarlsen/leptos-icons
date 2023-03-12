#[cfg(feature = "RiSystemFillArrowRightUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowRightUp")]
/// *This icon requires the feature* `RiSystemFillArrowRightUp` *to be enabled*.
#[component]
pub fn ArrowRightUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M13.05 12.36l-5.656 5.658-1.414-1.415 5.657-5.656-4.95-4.95H18V17.31z" /></g></svg>
   }
}