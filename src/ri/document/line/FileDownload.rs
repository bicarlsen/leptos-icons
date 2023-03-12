#[cfg(feature = "RiDocumentLineFileDownload")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentLineFileDownload")]
/// *This icon requires the feature* `RiDocumentLineFileDownload` *to be enabled*.
#[component]
pub fn FileDownload(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path fill-rule="nonzero" d="M13 12h3l-4 4-4-4h3V8h2v4zm2-8H5v16h14V8h-4V4zM3 2.992C3 2.444 3.447 2 3.999 2H16l5 5v13.993A1 1 0 0 1 20.007 22H3.993A1 1 0 0 1 3 21.008V2.992z" /></g></svg>
   }
}