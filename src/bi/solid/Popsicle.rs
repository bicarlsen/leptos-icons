#[cfg(feature = "BiSolidPopsicle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPopsicle")]
/// *This icon requires the feature* `BiSolidPopsicle` *to be enabled*.
#[component]
pub fn Popsicle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 4a7 7 0 0 0-9.9 0l-7.82 7.82a1 1 0 0 0 0 1.41l3.54 3.54-3.54 3.53 1.42 1.42 3.53-3.54 3.54 3.54a1 1 0 0 0 1.41 0L20 13.94A7 7 0 0 0 20 4zm-2.7 2.7a3.33 3.33 0 0 0-4.6 0l-1.06-1.06a4.76 4.76 0 0 1 6.72 0z" /></svg>
   }
}