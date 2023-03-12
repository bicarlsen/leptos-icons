#[cfg(feature = "BiSolidSkipPreviousCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidSkipPreviousCircle")]
/// *This icon requires the feature* `BiSolidSkipPreviousCircle` *to be enabled*.
#[component]
pub fn SkipPreviousCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10c5.515 0 10-4.486 10-10S17.515 2 12 2zm4 14-6-4v4H8V8h2v4l6-4v8z" /></svg>
   }
}