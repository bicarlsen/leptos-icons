#[cfg(feature = "BiRegularWindowAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularWindowAlt")]
/// *This icon requires the feature* `BiRegularWindowAlt` *to be enabled*.
#[component]
pub fn WindowAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 3H4c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zm0 2 .001 4H4V5h16zM4 19v-8h16.001l.001 8H4z" /><path d="M14 6h2v2h-2zm3 0h2v2h-2z" /></svg>
   }
}