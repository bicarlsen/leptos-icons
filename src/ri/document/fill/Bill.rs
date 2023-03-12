#[cfg(feature = "RiDocumentFillBill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentFillBill")]
/// *This icon requires the feature* `RiDocumentFillBill` *to be enabled*.
#[component]
pub fn Bill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M20 22H4a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1zM8 9v2h8V9H8zm0 4v2h8v-2H8z" /></g></svg>
   }
}