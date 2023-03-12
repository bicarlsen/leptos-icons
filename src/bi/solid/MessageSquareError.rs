#[cfg(feature = "BiSolidMessageSquareError")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageSquareError")]
/// *This icon requires the feature* `BiSolidMessageSquareError` *to be enabled*.
#[component]
pub fn MessageSquareError(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16 2H8C4.691 2 2 4.691 2 8v13a1 1 0 0 0 1 1h13c3.309 0 6-2.691 6-6V8c0-3.309-2.691-6-6-6zm-3 16h-2v-2h2v2zm0-4h-2V6h2v8z" /></svg>
   }
}