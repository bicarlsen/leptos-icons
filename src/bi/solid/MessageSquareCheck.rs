#[cfg(feature = "BiSolidMessageSquareCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageSquareCheck")]
/// *This icon requires the feature* `BiSolidMessageSquareCheck` *to be enabled*.
#[component]
pub fn MessageSquareCheck(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16 2H8C4.691 2 2 4.691 2 8v13a1 1 0 0 0 1 1h13c3.309 0 6-2.691 6-6V8c0-3.309-2.691-6-6-6zm-5 14.414-3.707-3.707 1.414-1.414L11 13.586l4.793-4.793 1.414 1.414L11 16.414z" /></svg>
   }
}