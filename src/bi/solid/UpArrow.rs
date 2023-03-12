#[cfg(feature = "BiSolidUpArrow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidUpArrow")]
/// *This icon requires the feature* `BiSolidUpArrow` *to be enabled*.
#[component]
pub fn UpArrow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 19h18a1.002 1.002 0 0 0 .823-1.569l-9-13c-.373-.539-1.271-.539-1.645 0l-9 13A.999.999 0 0 0 3 19z" /></svg>
   }
}