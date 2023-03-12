#[cfg(feature = "BiSolidChevronRightCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChevronRightCircle")]
/// *This icon requires the feature* `BiSolidChevronRightCircle` *to be enabled*.
#[component]
pub fn ChevronRightCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm-1.293 15.707-1.414-1.414L13.586 12 9.293 7.707l1.414-1.414L16.414 12l-5.707 5.707z" /></svg>
   }
}