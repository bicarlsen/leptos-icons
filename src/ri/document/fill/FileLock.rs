#[cfg(feature = "RiDocumentFillFileLock")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDocumentFillFileLock")]
/// *This icon requires the feature* `RiDocumentFillFileLock` *to be enabled*.
#[component]
pub fn FileLock(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M16 2l5 5v14.008a.993.993 0 0 1-.993.992H3.993A1 1 0 0 1 3 21.008V2.992C3 2.444 3.445 2 3.993 2H16zm-1 9v-1a3 3 0 0 0-6 0v1H8v5h8v-5h-1zm-2 0h-2v-1a1 1 0 0 1 2 0v1z" /></g></svg>
   }
}