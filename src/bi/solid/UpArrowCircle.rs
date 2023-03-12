#[cfg(feature = "BiSolidUpArrowCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidUpArrowCircle")]
/// *This icon requires the feature* `BiSolidUpArrowCircle` *to be enabled*.
#[component]
pub fn UpArrowCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 22c5.514 0 10-4.486 10-10S17.514 2 12 2 2 6.486 2 12s4.486 10 10 10zm0-15 5 5h-4v5h-2v-5H7l5-5z" /></svg>
   }
}