#[cfg(feature = "BiSolidNote")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidNote")]
/// *This icon requires the feature* `BiSolidNote` *to be enabled*.
#[component]
pub fn Note(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8l8-8V5a2 2 0 0 0-2-2zm-7 16v-7h7l-7 7z" /></svg>
   }
}