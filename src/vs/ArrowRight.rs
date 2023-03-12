#[cfg(feature = "VsArrowRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsArrowRight")]
/// *This icon requires the feature* `VsArrowRight` *to be enabled*.
#[component]
pub fn ArrowRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M9 13.887l5-5V8.18l-5-5-.707.707 4.146 4.147H2v1h10.44L8.292 13.18l.707.707z" /></svg>
   }
}