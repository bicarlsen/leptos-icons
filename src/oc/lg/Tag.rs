#[cfg(feature = "OcLgTag")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgTag")]
/// *This icon requires the feature* `OcLgTag` *to be enabled*.
#[component]
pub fn Tag(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7.75 6.5a1.25 1.25 0 1 0 0 2.5 1.25 1.25 0 0 0 0-2.5Z" /><path d="M2.5 1h8.44a1.5 1.5 0 0 1 1.06.44l10.25 10.25a1.5 1.5 0 0 1 0 2.12l-8.44 8.44a1.5 1.5 0 0 1-2.12 0L1.44 12A1.497 1.497 0 0 1 1 10.94V2.5A1.5 1.5 0 0 1 2.5 1Zm0 1.5v8.44l10.25 10.25 8.44-8.44L10.94 2.5Z" /></svg>
   }
}