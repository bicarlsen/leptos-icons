#[cfg(feature = "RiMediaFillRepeat")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaFillRepeat")]
/// *This icon requires the feature* `RiMediaFillRepeat` *to be enabled*.
#[component]
pub fn Repeat(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M6 4h15a1 1 0 0 1 1 1v7h-2V6H6v3L1 5l5-4v3zm12 16H3a1 1 0 0 1-1-1v-7h2v6h14v-3l5 4-5 4v-3z" /></g></svg>
   }
}