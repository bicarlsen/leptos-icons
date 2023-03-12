#[cfg(feature = "BiSolidUpArrowSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidUpArrowSquare")]
/// *This icon requires the feature* `BiSolidUpArrowSquare` *to be enabled*.
#[component]
pub fn UpArrowSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 21h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2zm7-14 5 5h-4v5h-2v-5H7l5-5z" /></svg>
   }
}