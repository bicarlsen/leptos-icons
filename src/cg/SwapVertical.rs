#[cfg(feature = "CgSwapVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSwapVertical")]
/// *This icon requires the feature* `CgSwapVertical` *to be enabled*.
#[component]
pub fn SwapVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12 16H13.5L13.5 10H15.5L15.5 16H17L14.5 19L12 16Z" fill="currentColor" /><path d="M8 8H9.5L9.5 14H11.5L11.5 8H13L10.5 5L8 8Z" fill="currentColor" /></svg>
   }
}