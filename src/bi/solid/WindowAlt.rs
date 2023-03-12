#[cfg(feature = "BiSolidWindowAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidWindowAlt")]
/// *This icon requires the feature* `BiSolidWindowAlt` *to be enabled*.
#[component]
pub fn WindowAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 3H4c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zm-3 3h2v2h-2V6zm-3 0h2v2h-2V6zM4 19v-9h16.001l.001 9H4z" /></svg>
   }
}