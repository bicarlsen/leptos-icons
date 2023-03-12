#[cfg(feature = "BiSolidLeftArrowSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidLeftArrowSquare")]
/// *This icon requires the feature* `BiSolidLeftArrowSquare` *to be enabled*.
#[component]
pub fn LeftArrowSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 21a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14zM12 7v4h5v2h-5v4l-5-5 5-5z" /></svg>
   }
}