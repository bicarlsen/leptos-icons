#[cfg(feature = "RiEditorSortAsc")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorSortAsc")]
/// *This icon requires the feature* `RiEditorSortAsc` *to be enabled*.
#[component]
pub fn SortAsc(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M19 3l4 5h-3v12h-2V8h-3l4-5zm-5 15v2H3v-2h11zm0-7v2H3v-2h11zm-2-7v2H3V4h9z" /></g></svg>
   }
}