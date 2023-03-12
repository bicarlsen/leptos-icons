#[cfg(feature = "BiRegularDockBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDockBottom")]
/// *This icon requires the feature* `BiRegularDockBottom` *to be enabled*.
#[component]
pub fn DockBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2zm0 2v9H5V5zM5 19v-3h14v3z" /></svg>
   }
}