#[cfg(feature = "BiSolidCaretRightCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCaretRightCircle")]
/// *This icon requires the feature* `BiSolidCaretRightCircle` *to be enabled*.
#[component]
pub fn CaretRightCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 22c5.514 0 10-4.486 10-10S17.514 2 12 2 2 6.486 2 12s4.486 10 10 10zM10 7l6 5-6 5V7z" /></svg>
   }
}