#[cfg(feature = "BiLogosUnsplash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosUnsplash")]
/// *This icon requires the feature* `BiLogosUnsplash` *to be enabled*.
#[component]
pub fn Unsplash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8.625 8.063V3h6.75v5.063h-6.75zm6.75 2.812H21V21H3V10.875h5.625v5.063h6.75v-5.063z" /></svg>
   }
}