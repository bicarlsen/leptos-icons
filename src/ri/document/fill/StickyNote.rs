#[cfg(feature = "RiDocumentFillStickyNote")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentFillStickyNote")]
/// *This icon requires the feature* `RiDocumentFillStickyNote` *to be enabled*.
#[component]
pub fn StickyNote(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M15 14l-.117.007a1 1 0 0 0-.876.876L14 15v6H3.998A.996.996 0 0 1 3 20.007V3.993C3 3.445 3.445 3 3.993 3h16.014c.548 0 .993.447.993.999V14h-6zm6 2l-5 4.997V16h5z" /></g></svg>
   }
}