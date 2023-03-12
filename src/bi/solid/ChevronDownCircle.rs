#[cfg(feature = "BiSolidChevronDownCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChevronDownCircle")]
/// *This icon requires the feature* `BiSolidChevronDownCircle` *to be enabled*.
#[component]
pub fn ChevronDownCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 14.414-5.707-5.707 1.414-1.414L12 13.586l4.293-4.293 1.414 1.414L12 16.414z" /></svg>
   }
}