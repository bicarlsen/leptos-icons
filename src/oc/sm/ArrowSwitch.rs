#[cfg(feature = "OcSmArrowSwitch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmArrowSwitch")]
/// *This icon requires the feature* `OcSmArrowSwitch` *to be enabled*.
#[component]
pub fn ArrowSwitch(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M5.22 14.78a.75.75 0 0 0 1.06-1.06L4.56 12h8.69a.75.75 0 0 0 0-1.5H4.56l1.72-1.72a.75.75 0 0 0-1.06-1.06l-3 3a.75.75 0 0 0 0 1.06l3 3Zm5.56-6.5a.75.75 0 1 1-1.06-1.06l1.72-1.72H2.75a.75.75 0 0 1 0-1.5h8.69L9.72 2.28a.75.75 0 0 1 1.06-1.06l3 3a.75.75 0 0 1 0 1.06l-3 3Z" /></svg>
   }
}