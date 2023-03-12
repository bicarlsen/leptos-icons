#[cfg(feature = "BiSolidArrowToRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidArrowToRight")]
/// *This icon requires the feature* `BiSolidArrowToRight` *to be enabled*.
#[component]
pub fn ArrowToRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 6h2v12h-2zm-8 5H4v2h6v5l6-6-6-6z" /></svg>
   }
}