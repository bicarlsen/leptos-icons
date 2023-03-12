#[cfg(feature = "RiEditorBringToFront")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorBringToFront")]
/// *This icon requires the feature* `RiEditorBringToFront` *to be enabled*.
#[component]
pub fn BringToFront(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M11 3c.552 0 1 .448 1 1v2h5c.552 0 1 .448 1 1v5h2c.552 0 1 .448 1 1v7c0 .552-.448 1-1 1h-7c-.552 0-1-.448-1-1v-2H7c-.552 0-1-.448-1-1v-5H4c-.552 0-1-.448-1-1V4c0-.552.448-1 1-1h7zm5 5H8v8h8V8z" /></g></svg>
   }
}