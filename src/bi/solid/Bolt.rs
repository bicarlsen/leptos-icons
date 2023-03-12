#[cfg(feature = "BiSolidBolt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBolt")]
/// *This icon requires the feature* `BiSolidBolt` *to be enabled*.
#[component]
pub fn Bolt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17.168 8H13l.806-4.835A1 1 0 0 0 12.819 2H7.667a1 1 0 0 0-.986.835l-1.667 10A1 1 0 0 0 6 14h4v8l8.01-12.459A1 1 0 0 0 17.168 8z" /></svg>
   }
}