#[cfg(feature = "BiRegularArea")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularArea")]
/// *This icon requires the feature* `BiRegularArea` *to be enabled*.
#[component]
pub fn Area(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 5v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2zm16.002 14H5V5h14l.002 14z" /><path d="M15 12h2V7h-5v2h3zm-3 3H9v-3H7v5h5z" /></svg>
   }
}