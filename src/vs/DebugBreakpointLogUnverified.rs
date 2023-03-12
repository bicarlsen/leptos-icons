#[cfg(feature = "VsDebugBreakpointLogUnverified")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugBreakpointLogUnverified")]
/// *This icon requires the feature* `VsDebugBreakpointLogUnverified` *to be enabled*.
#[component]
pub fn DebugBreakpointLogUnverified(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M3.02 7.98L8 3l4.98 4.98L8 12.96 3.02 7.98zM8 10.77l2.79-2.79L8 5.19 5.21 7.98 8 10.77z" /></svg>
   }
}