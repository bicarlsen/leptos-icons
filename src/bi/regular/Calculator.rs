#[cfg(feature = "BiRegularCalculator")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCalculator")]
/// *This icon requires the feature* `BiRegularCalculator` *to be enabled*.
#[component]
pub fn Calculator(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 2H5c-1.103 0-2 .897-2 2v16c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zM5 20V4h14l.001 16H5z" /><path d="M7 12h2v2H7zm0 4h2v2H7zm4-4h2v2h-2zM7 6h10v4H7zm4 10h2v2h-2zm4-4h2v6h-2z" /></svg>
   }
}