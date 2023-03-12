#[cfg(feature = "BiRegularTrim")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularTrim")]
/// *This icon requires the feature* `BiRegularTrim` *to be enabled*.
#[component]
pub fn Trim(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 16h3v3c0 1.103.897 2 2 2h9c1.103 0 2-.897 2-2v-9c0-1.103-.897-2-2-2h-3V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v9c0 1.103.897 2 2 2zm14-6-.003 9H10v-3h4c1.103 0 2-.897 2-2v-4h3zM5 10h5V5h4l-.003 9H5v-4z" /></svg>
   }
}