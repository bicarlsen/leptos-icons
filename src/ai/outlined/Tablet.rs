#[cfg(feature = "AiOutlinedTablet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedTablet")]
/// *This icon requires the feature* `AiOutlinedTablet` *to be enabled*.
#[component]
pub fn Tablet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M800 64H224c-35.3 0-64 28.7-64 64v768c0 35.3 28.7 64 64 64h576c35.3 0 64-28.7 64-64V128c0-35.3-28.7-64-64-64zm-8 824H232V136h560v752zM472 784a40 40 0 1 0 80 0 40 40 0 1 0-80 0z" /></svg>
   }
}