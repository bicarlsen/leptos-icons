#[cfg(feature = "BiRegularMessageAltCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMessageAltCheck")]
/// *This icon requires the feature* `BiRegularMessageAltCheck` *to be enabled*.
#[component]
pub fn MessageAltCheck(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 4v12c0 1.103.897 2 2 2h3.586L12 21.414 15.414 18H19c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2zm2 0h14v12h-4.414L12 18.586 9.414 16H5V4z" /><path d="m17.207 7.207-1.414-1.414L11 10.586 8.707 8.293 7.293 9.707 11 13.414z" /></svg>
   }
}