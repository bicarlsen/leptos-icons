#[cfg(feature = "RiEditorSortDesc")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorSortDesc")]
/// *This icon requires the feature* `RiEditorSortDesc` *to be enabled*.
#[component]
pub fn SortDesc(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M20 4v12h3l-4 5-4-5h3V4h2zm-8 14v2H3v-2h9zm2-7v2H3v-2h11zm0-7v2H3V4h11z" /></g></svg>
   }
}