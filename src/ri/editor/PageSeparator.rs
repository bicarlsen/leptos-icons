#[cfg(feature = "RiEditorPageSeparator")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorPageSeparator")]
/// *This icon requires the feature* `RiEditorPageSeparator` *to be enabled*.
#[component]
pub fn PageSeparator(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M17 21v-4H7v4H5v-5a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v5h-2zM7 3v4h10V3h2v5a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1V3h2zM2 9l4 3-4 3V9zm20 0v6l-4-3 4-3z" /></g></svg>
   }
}