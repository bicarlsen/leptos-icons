#[cfg(feature = "BiRegularWindow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularWindow")]
/// *This icon requires the feature* `BiRegularWindow` *to be enabled*.
#[component]
pub fn Window(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 21h16c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2H4c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2zm0-2V7h16l.001 12H4z" /></svg>
   }
}