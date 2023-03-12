#[cfg(feature = "HiLgOutlinePause")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlinePause")]
/// *This icon requires the feature* `HiLgOutlinePause` *to be enabled*.
#[component]
pub fn Pause(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M15.75 5.25L15.75 18.75M8.25 5.25V18.75" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}