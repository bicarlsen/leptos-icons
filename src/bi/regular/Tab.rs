#[cfg(feature = "BiRegularTab")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularTab")]
/// *This icon requires the feature* `BiRegularTab` *to be enabled*.
#[component]
pub fn Tab(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 2c-1.103 0-2 .897-2 2v16c0 1.103.897 2 2 2h12c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2H6zm0 15V5h12l.002 12H6z" /></svg>
   }
}