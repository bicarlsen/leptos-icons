#[cfg(feature = "CgPushUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPushUp")]
/// *This icon requires the feature* `CgPushUp` *to be enabled*.
#[component]
pub fn PushUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M11.0001 22.2877H13.0001V7.80237L16.2428 11.045L17.657 9.63079L12.0001 3.97394L6.34326 9.63079L7.75748 11.045L11.0001 7.80236V22.2877ZM18 3H6V1H18V3Z" fill="currentColor" /></svg>
   }
}