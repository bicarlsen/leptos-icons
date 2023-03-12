#[cfg(feature = "RiEditorListCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiEditorListCheck")]
/// *This icon requires the feature* `RiEditorListCheck` *to be enabled*.
#[component]
pub fn ListCheck(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M8 4h13v2H8V4zm-5-.5h3v3H3v-3zm0 7h3v3H3v-3zm0 7h3v3H3v-3zM8 11h13v2H8v-2zm0 7h13v2H8v-2z" /></g></svg>
   }
}