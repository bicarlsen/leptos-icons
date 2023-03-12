#[cfg(feature = "BiRegularUnderline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularUnderline")]
/// *This icon requires the feature* `BiRegularUnderline` *to be enabled*.
#[component]
pub fn Underline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 18h14v2H5zM6 4v6c0 3.309 2.691 6 6 6s6-2.691 6-6V4h-2v6c0 2.206-1.794 4-4 4s-4-1.794-4-4V4H6z" /></svg>
   }
}