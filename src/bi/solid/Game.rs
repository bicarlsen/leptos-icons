#[cfg(feature = "BiSolidGame")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidGame")]
/// *This icon requires the feature* `BiSolidGame` *to be enabled*.
#[component]
pub fn Game(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 22c3.719 0 7.063-2.035 8.809-5.314L13 12l7.809-4.686C19.063 4.035 15.719 2 12 2 6.486 2 2 6.486 2 12s4.486 10 10 10zm-.5-16a1.5 1.5 0 1 1-.001 3.001A1.5 1.5 0 0 1 11.5 6z" /></svg>
   }
}