#[cfg(feature = "OcLgAlertFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgAlertFill")]
/// *This icon requires the feature* `OcLgAlertFill` *to be enabled*.
#[component]
pub fn AlertFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M9.836 3.244c.963-1.665 3.365-1.665 4.328 0l8.967 15.504c.963 1.667-.24 3.752-2.165 3.752H3.034c-1.926 0-3.128-2.085-2.165-3.752ZM12 8.5a.75.75 0 0 0-.75.75v4.5a.75.75 0 0 0 1.5 0v-4.5A.75.75 0 0 0 12 8.5Zm1 9a1 1 0 1 0-2 0 1 1 0 0 0 2 0Z" /></svg>
   }
}