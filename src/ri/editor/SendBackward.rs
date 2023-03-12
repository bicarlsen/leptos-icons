#[cfg(feature = "RiEditorSendBackward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorSendBackward")]
/// *This icon requires the feature* `RiEditorSendBackward` *to be enabled*.
#[component]
pub fn SendBackward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M14 3c.552 0 1 .448 1 1v5h5c.552 0 1 .448 1 1v10c0 .552-.448 1-1 1H10c-.552 0-1-.448-1-1v-5H4c-.552 0-1-.448-1-1V4c0-.552.448-1 1-1h10zm-1 2H5v8h4v-3c0-.552.448-1 1-1h3V5z" /></g></svg>
   }
}