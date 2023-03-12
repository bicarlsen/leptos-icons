#[cfg(feature = "RiDocumentLineFileShred")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentLineFileShred")]
/// *This icon requires the feature* `RiDocumentLineFileShred` *to be enabled*.
#[component]
pub fn FileShred(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M6 12h12V8h-4V4H6v8zm-2 0V2.995c0-.55.445-.995.996-.995H15l5 5v5h2v2H2v-2h2zm-1 4h2v6H3v-6zm16 0h2v6h-2v-6zm-4 0h2v6h-2v-6zm-4 0h2v6h-2v-6zm-4 0h2v6H7v-6z" /></g></svg>
   }
}