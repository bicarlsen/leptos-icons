#[cfg(feature = "RiDeviceFillTv")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillTv")]
/// *This icon requires the feature* `RiDeviceFillTv` *to be enabled*.
#[component]
pub fn Tv(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M15.414 5h5.594c.548 0 .992.445.992 1v14c0 .552-.455 1-.992 1H2.992A.994.994 0 0 1 2 20V6c0-.552.455-1 .992-1h5.594L6.05 2.464 7.464 1.05 11.414 5h1.172l3.95-3.95 1.414 1.414L15.414 5z" /></g></svg>
   }
}