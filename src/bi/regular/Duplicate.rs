#[cfg(feature = "BiRegularDuplicate")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDuplicate")]
/// *This icon requires the feature* `BiRegularDuplicate` *to be enabled*.
#[component]
pub fn Duplicate(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11 10H9v3H6v2h3v3h2v-3h3v-2h-3z" /><path d="M4 22h12c1.103 0 2-.897 2-2V8c0-1.103-.897-2-2-2H4c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2zM4 8h12l.002 12H4V8z" /><path d="M20 2H8v2h12v12h2V4c0-1.103-.897-2-2-2z" /></svg>
   }
}