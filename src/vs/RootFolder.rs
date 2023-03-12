#[cfg(feature = "VsRootFolder")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsRootFolder")]
/// *This icon requires the feature* `VsRootFolder` *to be enabled*.
#[component]
pub fn RootFolder(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M7.71 3h6.79l.51.5v10l-.5.5H8.743a5.48 5.48 0 0 0 .657-1h4.59v-1.51l.01-4v-1.5H7.69l-.017.017a5.494 5.494 0 0 0-.881-.508l.348-.349.35-.15h6.5l.01-.99H7.5l-.36-.15-.85-.85H2V5.6a5.45 5.45 0 0 0-.99.649V2.5l.5-.5h5l.35.15.85.85z" /><path d="M6 10.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0z" /><path fill-rule="evenodd" clip-rule="evenodd" d="M8 10.5a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0zM4.5 13a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5z" /></svg>
   }
}