#[cfg(feature = "VsArrowSwap")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsArrowSwap")]
/// *This icon requires the feature* `VsArrowSwap` *to be enabled*.
#[component]
pub fn ArrowSwap(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M4.207 15.061L1 11.854v-.707L4.207 7.94l.707.707-2.353 2.354H15v1H2.56l2.354 2.353-.707.707zm7.586-7L15 4.854v-.707L11.793.94l-.707.707L13.439 4H1v1h12.44l-2.354 2.354.707.707z" /></svg>
   }
}