#[cfg(feature = "CgLayoutGrid")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgLayoutGrid")]
/// *This icon requires the feature* `CgLayoutGrid` *to be enabled*.
#[component]
pub fn LayoutGrid(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M11 7H7V11H11V7Z" fill="currentColor" /><path d="M11 13H7V17H11V13Z" fill="currentColor" /><path d="M13 13H17V17H13V13Z" fill="currentColor" /><path d="M17 7H13V11H17V7Z" fill="currentColor" /></svg>
   }
}