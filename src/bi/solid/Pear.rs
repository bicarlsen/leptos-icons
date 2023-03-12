#[cfg(feature = "BiSolidPear")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPear")]
/// *This icon requires the feature* `BiSolidPear` *to be enabled*.
#[component]
pub fn Pear(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10.12 22a7.71 7.71 0 0 0 6.57-5 7.23 7.23 0 0 0 .46-3.21 3.26 3.26 0 0 1 1-2.57l.61-.61A3.89 3.89 0 0 0 19.43 6l2.28-2.28-1.42-1.43L18 4.55a3.82 3.82 0 0 0-4.66.57l-.75.75a3.22 3.22 0 0 1-2.52 1 7.05 7.05 0 0 0-3.32.57A7.75 7.75 0 0 0 2 14.11 7.59 7.59 0 0 0 10.12 22zM9.5 9.25v1.5a3.75 3.75 0 0 0-3.75 3.75h-1.5A5.26 5.26 0 0 1 9.5 9.25z" /></svg>
   }
}