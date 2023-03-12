#[cfg(feature = "IoCaretBackOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCaretBackOutline")]
/// *This icon requires the feature* `IoCaretBackOutline` *to be enabled*.
#[component]
pub fn CaretBackOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M321.94,98,158.82,237.78a24,24,0,0,0,0,36.44L321.94,414c15.57,13.34,39.62,2.28,39.62-18.22V116.18C361.56,95.68,337.51,84.62,321.94,98Z" /></svg>
   }
}