#[cfg(feature = "CgCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgCheck")]
/// *This icon requires the feature* `CgCheck` *to be enabled*.
#[component]
pub fn Check(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M10.5858 13.4142L7.75735 10.5858L6.34314 12L10.5858 16.2427L17.6568 9.1716L16.2426 7.75739L10.5858 13.4142Z" fill="currentColor" /></svg>
   }
}