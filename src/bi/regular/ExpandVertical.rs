#[cfg(feature = "BiRegularExpandVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularExpandVertical")]
/// *This icon requires the feature* `BiRegularExpandVertical` *to be enabled*.
#[component]
pub fn ExpandVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m12 19.24-4.95-4.95-1.41 1.42L12 22.07l6.36-6.36-1.41-1.42L12 19.24zM5.64 8.29l1.41 1.42L12 4.76l4.95 4.95 1.41-1.42L12 1.93 5.64 8.29z" /></svg>
   }
}