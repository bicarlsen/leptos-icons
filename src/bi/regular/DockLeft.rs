#[cfg(feature = "BiRegularDockLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDockLeft")]
/// *This icon requires the feature* `BiRegularDockLeft` *to be enabled*.
#[component]
pub fn DockLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2zM5 5h3v14H5zm5 14V5h9v14z" /></svg>
   }
}