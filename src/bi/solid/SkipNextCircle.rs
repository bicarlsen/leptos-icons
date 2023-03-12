#[cfg(feature = "BiSolidSkipNextCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidSkipNextCircle")]
/// *This icon requires the feature* `BiSolidSkipNextCircle` *to be enabled*.
#[component]
pub fn SkipNextCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm4 14h-2v-4l-6 4V8l6 4V8h2v8z" /></svg>
   }
}