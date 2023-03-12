#[cfg(feature = "RiDocumentFillDraft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentFillDraft")]
/// *This icon requires the feature* `RiDocumentFillDraft` *to be enabled*.
#[component]
pub fn Draft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0L24 0 24 24 0 24z" /><path d="M20 2c.552 0 1 .448 1 1v3.757l-8.999 9-.006 4.238 4.246.006L21 15.242V21c0 .552-.448 1-1 1H4c-.552 0-1-.448-1-1V3c0-.552.448-1 1-1h16zm1.778 6.808l1.414 1.414L15.414 18l-1.416-.002.002-1.412 7.778-7.778zM12 12H7v2h5v-2zm3-4H7v2h8V8z" /></g></svg>
   }
}