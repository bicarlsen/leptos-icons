#[cfg(feature = "VsDebugBreakpointConditional")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugBreakpointConditional")]
/// *This icon requires the feature* `VsDebugBreakpointConditional` *to be enabled*.
#[component]
pub fn DebugBreakpointConditional(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M8 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8zm2 5v1H6V9h4zm0-3v1H6V6h4z" /></svg>
   }
}