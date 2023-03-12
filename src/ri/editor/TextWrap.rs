#[cfg(feature = "RiEditorTextWrap")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorTextWrap")]
/// *This icon requires the feature* `RiEditorTextWrap` *to be enabled*.
#[component]
pub fn TextWrap(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M15 18h1.5a2.5 2.5 0 1 0 0-5H3v-2h13.5a4.5 4.5 0 1 1 0 9H15v2l-4-3 4-3v2zM3 4h18v2H3V4zm6 14v2H3v-2h6z" /></g></svg>
   }
}