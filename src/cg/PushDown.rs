#[cfg(feature = "CgPushDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPushDown")]
/// *This icon requires the feature* `CgPushDown` *to be enabled*.
#[component]
pub fn PushDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M11.0001 1H13.0001V15.4853L16.2428 12.2427L17.657 13.6569L12.0001 19.3137L6.34326 13.6569L7.75748 12.2427L11.0001 15.4853V1Z" fill="currentColor" /><path d="M18 20.2877H6V22.2877H18V20.2877Z" fill="currentColor" /></svg>
   }
}