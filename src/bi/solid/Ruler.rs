#[cfg(feature = "BiSolidRuler")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidRuler")]
/// *This icon requires the feature* `BiSolidRuler` *to be enabled*.
#[component]
pub fn Ruler(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20.875 7H3.125C1.953 7 1 7.897 1 9v6c0 1.103.953 2 2.125 2h17.75C22.047 17 23 16.103 23 15V9c0-1.103-.953-2-2.125-2zM7 12H5V9h2v3zm4 1H9V9h2v4zm4-1h-2V9h2v3zm4 1h-2V9h2v4z" /></svg>
   }
}