#[cfg(feature = "CgSize")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSize")]
/// *This icon requires the feature* `CgSize` *to be enabled*.
#[component]
pub fn Size(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M12 6V4H20V20H12V18H8V16H4V8H8V6H12ZM14 6H18V18H14V6ZM12 8H10V16H12V8ZM8 10V14H6V10H8Z" fill="currentColor" /></svg>
   }
}