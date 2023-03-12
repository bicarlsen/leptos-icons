#[cfg(feature = "HiLgOutlineViewColumns")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineViewColumns")]
/// *This icon requires the feature* `HiLgOutlineViewColumns` *to be enabled*.
#[component]
pub fn ViewColumns(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M9 4.5V19.5M15 4.5V19.5M4.125 19.5H19.875C20.4963 19.5 21 18.9963 21 18.375V5.625C21 5.00368 20.4963 4.5 19.875 4.5H4.125C3.50368 4.5 3 5.00368 3 5.625V18.375C3 18.9963 3.50368 19.5 4.125 19.5Z" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}