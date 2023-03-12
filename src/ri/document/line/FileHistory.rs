#[cfg(feature = "RiDocumentLineFileHistory")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentLineFileHistory")]
/// *This icon requires the feature* `RiDocumentLineFileHistory` *to be enabled*.
#[component]
pub fn FileHistory(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0L24 0 24 24 0 24z" /><path d="M16 2l5 5v13.993c0 .556-.445 1.007-.993 1.007H3.993C3.445 22 3 21.545 3 21.008V2.992C3 2.444 3.447 2 3.999 2H16zm-1 2H5v16h14V8h-4V4zm-2 5v4h3v2h-5V9h2z" /></g></svg>
   }
}