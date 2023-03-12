#[cfg(feature = "BiSolidCaretLeftCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCaretLeftCircle")]
/// *This icon requires the feature* `BiSolidCaretLeftCircle` *to be enabled*.
#[component]
pub fn CaretLeftCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm2 15-6-5 6-5v10z" /></svg>
   }
}