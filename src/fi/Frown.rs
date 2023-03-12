#[cfg(feature = "FiFrown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiFrown")]
/// *This icon requires the feature* `FiFrown` *to be enabled*.
#[component]
pub fn Frown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><path d="M16 16s-1.5-2-4-2-4 2-4 2" /><line x1="9" y1="9" x2="9.01" y2="9" /><line x1="15" y1="9" x2="15.01" y2="9" /></svg>
   }
}