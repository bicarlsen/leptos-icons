#[cfg(feature = "CgPathCrop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPathCrop")]
/// *This icon requires the feature* `CgPathCrop` *to be enabled*.
#[component]
pub fn PathCrop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><rect opacity="0.5" x="6" y="6" width="8" height="8" stroke="currentColor" stroke-width="2" /><path fill-rule="evenodd" clip-rule="evenodd" d="M9 9H19V19H9V9ZM15 11H17V17H11V15H15V11Z" fill="currentColor" /></svg>
   }
}