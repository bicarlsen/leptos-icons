#[cfg(feature = "BiSolidRightArrowSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidRightArrowSquare")]
/// *This icon requires the feature* `BiSolidRightArrowSquare` *to be enabled*.
#[component]
pub fn RightArrowSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 5v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2zm4 6h5V7l5 5-5 5v-4H7v-2z" /></svg>
   }
}