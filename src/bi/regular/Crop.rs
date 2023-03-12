#[cfg(feature = "BiRegularCrop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCrop")]
/// *This icon requires the feature* `BiRegularCrop` *to be enabled*.
#[component]
pub fn Crop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 7c0-1.103-.897-2-2-2H7V2H5v3H2v2h15v15h2v-3h3v-2h-3V7z" /><path d="M5 9v8c0 1.103.897 2 2 2h8v-2H7V9H5z" /></svg>
   }
}