#[cfg(feature = "BiSolidMagnet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMagnet")]
/// *This icon requires the feature* `BiSolidMagnet` *to be enabled*.
#[component]
pub fn Magnet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8 3H5a1 1 0 0 0-1 1v3h5V4a1 1 0 0 0-1-1zm7 1v3h5V4a1 1 0 0 0-1-1h-3a1 1 0 0 0-1 1zm0 10a3 3 0 0 1-6 0V9H4v5a8 8 0 0 0 16 0V9h-5v5z" /></svg>
   }
}