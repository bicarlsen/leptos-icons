#[cfg(feature = "BiRegularMessageAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMessageAlt")]
/// *This icon requires the feature* `BiRegularMessageAlt` *to be enabled*.
#[component]
pub fn MessageAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 2H5c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h3.586L12 21.414 15.414 18H19c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm0 14h-4.414L12 18.586 9.414 16H5V4h14v12z" /></svg>
   }
}