#[cfg(feature = "VsDesktopDownload")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDesktopDownload")]
/// *This icon requires the feature* `VsDesktopDownload` *to be enabled*.
#[component]
pub fn DesktopDownload(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M4 15v-1c2 0 2-.6 2-1H1.5l-.5-.5v-10l.5-.5h13l.5.5v9.24l-1-1V3H2v9h5.73l-.5.5 2.5 2.5H4zm7.86 0l2.5-2.5-.71-.7L12 13.45V7h-1v6.44l-1.64-1.65-.71.71 2.5 2.5h.71z" /></svg>
   }
}