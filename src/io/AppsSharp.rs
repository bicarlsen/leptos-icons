#[cfg(feature = "IoAppsSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoAppsSharp")]
/// *This icon requires the feature* `IoAppsSharp` *to be enabled*.
#[component]
pub fn AppsSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="48" y="48" width="112" height="112" rx="8" ry="8" /><rect x="200" y="48" width="112" height="112" rx="8" ry="8" /><rect x="352" y="48" width="112" height="112" rx="8" ry="8" /><rect x="48" y="200" width="112" height="112" rx="8" ry="8" /><rect x="200" y="200" width="112" height="112" rx="8" ry="8" /><rect x="352" y="200" width="112" height="112" rx="8" ry="8" /><rect x="48" y="352" width="112" height="112" rx="8" ry="8" /><rect x="200" y="352" width="112" height="112" rx="8" ry="8" /><rect x="352" y="352" width="112" height="112" rx="8" ry="8" /></svg>
   }
}