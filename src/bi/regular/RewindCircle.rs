#[cfg(feature = "BiRegularRewindCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularRewindCircle")]
/// *This icon requires the feature* `BiRegularRewindCircle` *to be enabled*.
#[component]
pub fn RewindCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8s3.589-8 8-8 8 3.589 8 8-3.589 8-8 8z" /><path d="M11 16V8l-5 4zm6 0V8l-5 4z" /></svg>
   }
}