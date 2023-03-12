#[cfg(feature = "RiDocumentFillBookRead")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentFillBookRead")]
/// *This icon requires the feature* `RiDocumentFillBookRead` *to be enabled*.
#[component]
pub fn BookRead(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2 3.993A1 1 0 0 1 2.992 3h18.016c.548 0 .992.445.992.993v16.014a1 1 0 0 1-.992.993H2.992A.993.993 0 0 1 2 20.007V3.993zM12 5v14h8V5h-8zm1 2h6v2h-6V7zm0 3h6v2h-6v-2z" /></g></svg>
   }
}