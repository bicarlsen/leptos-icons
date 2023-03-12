#[cfg(feature = "BiRegularDockTop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDockTop")]
/// *This icon requires the feature* `BiRegularDockTop` *to be enabled*.
#[component]
pub fn DockTop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2zm0 2v3H5V5zM5 19v-9h14v9z" /></svg>
   }
}