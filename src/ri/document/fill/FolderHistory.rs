#[cfg(feature = "RiDocumentFillFolderHistory")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentFillFolderHistory")]
/// *This icon requires the feature* `RiDocumentFillFolderHistory` *to be enabled*.
#[component]
pub fn FolderHistory(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0L24 0 24 24 0 24z" /><path d="M10.414 3l2 2H21c.552 0 1 .448 1 1v14c0 .552-.448 1-1 1H3c-.552 0-1-.448-1-1V4c0-.552.448-1 1-1h7.414zM13 9h-2v6h5v-2h-3V9z" /></g></svg>
   }
}