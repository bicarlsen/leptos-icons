#[cfg(feature = "HiLgOutlineMoon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineMoon")]
/// *This icon requires the feature* `HiLgOutlineMoon` *to be enabled*.
#[component]
pub fn Moon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M21.7519 15.0019C20.597 15.4839 19.3296 15.75 18 15.75C12.6152 15.75 8.25 11.3848 8.25 5.99999C8.25 4.67039 8.51614 3.40296 8.99806 2.24805C5.47566 3.71785 3 7.19481 3 11.25C3 16.6348 7.36522 21 12.75 21C16.8052 21 20.2821 18.5243 21.7519 15.0019Z" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}