#[cfg(feature = "RiDocumentLineDraft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentLineDraft")]
/// *This icon requires the feature* `RiDocumentLineDraft` *to be enabled*.
#[component]
pub fn Draft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0L24 0 24 24 0 24z" /><path d="M20 2c.552 0 1 .448 1 1v3.757l-2 2V4H5v16h14v-2.758l2-2V21c0 .552-.448 1-1 1H4c-.552 0-1-.448-1-1V3c0-.552.448-1 1-1h16zm1.778 6.808l1.414 1.414L15.414 18l-1.416-.002.002-1.412 7.778-7.778zM13 12v2H8v-2h5zm3-4v2H8V8h8z" /></g></svg>
   }
}