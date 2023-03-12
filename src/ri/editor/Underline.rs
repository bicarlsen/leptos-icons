#[cfg(feature = "RiEditorUnderline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorUnderline")]
/// *This icon requires the feature* `RiEditorUnderline` *to be enabled*.
#[component]
pub fn Underline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M8 3v9a4 4 0 1 0 8 0V3h2v9a6 6 0 1 1-12 0V3h2zM4 20h16v2H4v-2z" /></g></svg>
   }
}