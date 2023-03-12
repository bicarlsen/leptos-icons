#[cfg(feature = "CgPentagonRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPentagonRight")]
/// *This icon requires the feature* `CgPentagonRight` *to be enabled*.
#[component]
pub fn PentagonRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M8 12L6 7H16L18 12L16 17H6L8 12ZM8.95407 15L10.1541 12L8.95407 9H14.6459L15.8459 12L14.6459 15H8.95407Z" fill="currentColor" /></svg>
   }
}