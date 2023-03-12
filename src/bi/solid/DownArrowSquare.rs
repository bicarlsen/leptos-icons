#[cfg(feature = "BiSolidDownArrowSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDownArrowSquare")]
/// *This icon requires the feature* `BiSolidDownArrowSquare` *to be enabled*.
#[component]
pub fn DownArrowSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 21a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14zm-8-9V7h2v5h4l-5 5-5-5h4z" /></svg>
   }
}