#[cfg(feature = "VsTerminal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsTerminal")]
/// *This icon requires the feature* `VsTerminal` *to be enabled*.
#[component]
pub fn Terminal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M3 1.5L1.5 3v18L3 22.5h18l1.5-1.5V3L21 1.5H3zM3 21V3h18v18H3zm5.656-4.01l1.038 1.061 5.26-5.243v-.912l-5.26-5.26-1.035 1.06 4.59 4.702-4.593 4.592z" /></svg>
   }
}