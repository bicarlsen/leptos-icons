#[cfg(feature = "VsTerminalTmux")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsTerminalTmux")]
/// *This icon requires the feature* `VsTerminalTmux` *to be enabled*.
#[component]
pub fn TerminalTmux(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M13.5 1h-12l-.5.5v13l.5.5h12l.5-.5v-13l-.5-.5zM7 7.5V13H2V2h5v5.5zm6 5.5H8V8h5v5zm0-6H8V2h5v5z" /></svg>
   }
}