#[cfg(feature = "BiSolidShapes")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidShapes")]
/// *This icon requires the feature* `BiSolidShapes` *to be enabled*.
#[component]
pub fn Shapes(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17.867 2.504c-.355-.624-1.381-.623-1.736 0l-3.999 7A1 1 0 0 0 13 11h8a1.001 1.001 0 0 0 .868-1.496l-4.001-7zM3 22h7a1 1 0 0 0 1-1v-7a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v7a1 1 0 0 0 1 1zm14.5-9c-2.481 0-4.5 2.019-4.5 4.5s2.019 4.5 4.5 4.5 4.5-2.019 4.5-4.5-2.019-4.5-4.5-4.5z" /></svg>
   }
}