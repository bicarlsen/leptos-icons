#[cfg(feature = "FaSolidMobileScreenButton")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidMobileScreenButton")]
/// *This icon requires the feature* `FaSolidMobileScreenButton` *to be enabled*.
#[component]
pub fn MobileScreenButton(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M0 64C0 28.7 28.7 0 64 0H288c35.3 0 64 28.7 64 64V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zM208 448a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zM288 64H64V384H288V64z" /></svg>
   }
}