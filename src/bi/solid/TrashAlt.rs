#[cfg(feature = "BiSolidTrashAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidTrashAlt")]
/// *This icon requires the feature* `BiSolidTrashAlt` *to be enabled*.
#[component]
pub fn TrashAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 7H5v13a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V7H6zm10.618-3L15 2H9L7.382 4H3v2h18V4z" /></svg>
   }
}