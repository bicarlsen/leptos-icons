#[cfg(feature = "BiRegularSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSquare")]
/// *This icon requires the feature* `BiRegularSquare` *to be enabled*.
#[component]
pub fn Square(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 3H4a1 1 0 0 0-1 1v16a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1V4a1 1 0 0 0-1-1zm-1 16H5V5h14v14z" /></svg>
   }
}