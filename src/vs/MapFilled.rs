#[cfg(feature = "VsMapFilled")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsMapFilled")]
/// *This icon requires the feature* `VsMapFilled` *to be enabled*.
#[component]
pub fn MapFilled(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M2 5.5V13L5.5 10.8125V3.3125L2 5.5Z" /><path d="M9.5 12.6875V5.1875L6.5 3.3125V10.8125L9.5 12.6875Z" /><path d="M10.5 12.6875V5.1875L14 3V10.5L10.5 12.6875Z" /></svg>
   }
}