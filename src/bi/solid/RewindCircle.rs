#[cfg(feature = "BiSolidRewindCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidRewindCircle")]
/// *This icon requires the feature* `BiSolidRewindCircle` *to be enabled*.
#[component]
pub fn RewindCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.485 2 12s4.486 10 10 10c5.515 0 10-4.485 10-10S17.515 2 12 2zm5 14-6-4v4l-6-4 6-4v4l6-4v8z" /></svg>
   }
}