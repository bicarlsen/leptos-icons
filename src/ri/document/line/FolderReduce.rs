#[cfg(feature = "RiDocumentLineFolderReduce")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentLineFolderReduce")]
/// *This icon requires the feature* `RiDocumentLineFolderReduce` *to be enabled*.
#[component]
pub fn FolderReduce(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12.414 5H21a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h7.414l2 2zM4 5v14h16V7h-8.414l-2-2H4zm4 7h8v2H8v-2z" /></g></svg>
   }
}