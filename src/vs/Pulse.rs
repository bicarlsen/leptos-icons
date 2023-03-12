#[cfg(feature = "VsPulse")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsPulse")]
/// *This icon requires the feature* `VsPulse` *to be enabled*.
#[component]
pub fn Pulse(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M11.8 9L10 3H9L7.158 9.64 5.99 4.69h-.97L3.85 9H1v.99h3.23l.49-.37.74-2.7L6.59 12h1.03l1.87-7.04 1.46 4.68.48.36H15V9h-3.2z" /></svg>
   }
}