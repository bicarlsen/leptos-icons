#[cfg(feature = "FiCrosshair")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCrosshair")]
/// *This icon requires the feature* `FiCrosshair` *to be enabled*.
#[component]
pub fn Crosshair(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><line x1="22" y1="12" x2="18" y2="12" /><line x1="6" y1="12" x2="2" y2="12" /><line x1="12" y1="6" x2="12" y2="2" /><line x1="12" y1="22" x2="12" y2="18" /></svg>
   }
}