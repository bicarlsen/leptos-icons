#[cfg(feature = "BiSolidFilterAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFilterAlt")]
/// *This icon requires the feature* `BiSolidFilterAlt` *to be enabled*.
#[component]
pub fn FilterAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13 20v-4.586L20.414 8c.375-.375.586-.884.586-1.415V4a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v2.585c0 .531.211 1.04.586 1.415L11 15.414V22l2-2z" /></svg>
   }
}