#[cfg(feature = "RiEditorSplitCellsVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorSplitCellsVertical")]
/// *This icon requires the feature* `RiEditorSplitCellsVertical` *to be enabled*.
#[component]
pub fn SplitCellsVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M20 3c.552 0 1 .448 1 1v16c0 .552-.448 1-1 1H4c-.552 0-1-.448-1-1V4c0-.552.448-1 1-1h16zm-1 2H5v5.999L9 11v2H5v6h14v-6h-4v-2l4-.001V5zm-7 1l3 3h-2v6h2l-3 3-3-3h2V9H9l3-3z" /></g></svg>
   }
}