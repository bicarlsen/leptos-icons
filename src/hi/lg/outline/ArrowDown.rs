#[cfg(feature = "HiLgOutlineArrowDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineArrowDown")]
/// *This icon requires the feature* `HiLgOutlineArrowDown` *to be enabled*.
#[component]
pub fn ArrowDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M19.5 13.5L12 21M12 21L4.5 13.5M12 21L12 3" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}