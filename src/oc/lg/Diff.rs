#[cfg(feature = "OcLgDiff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgDiff")]
/// *This icon requires the feature* `OcLgDiff` *to be enabled*.
#[component]
pub fn Diff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12.25 3.5a.75.75 0 0 1 .75.75V8.5h4.25a.75.75 0 0 1 0 1.5H13v4.25a.75.75 0 0 1-1.5 0V10H7.25a.75.75 0 0 1 0-1.5h4.25V4.25a.75.75 0 0 1 .75-.75ZM6.562 19.25a.75.75 0 0 1 .75-.75h9.938a.75.75 0 0 1 0 1.5H7.312a.75.75 0 0 1-.75-.75Z" /></svg>
   }
}