#[cfg(feature = "OcLgInfinity")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgInfinity")]
/// *This icon requires the feature* `OcLgInfinity` *to be enabled*.
#[component]
pub fn Infinity(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 11.16c.887-.933 1.813-1.865 2.78-2.6C15.952 7.668 17.267 7 18.75 7 21.657 7 24 9.615 24 12.25s-2.343 5.25-5.25 5.25c-1.483 0-2.798-.668-3.97-1.56-.967-.735-1.893-1.667-2.78-2.6-.887.933-1.813 1.865-2.78 2.6-1.172.892-2.487 1.56-3.97 1.56C2.343 17.5 0 14.885 0 12.25S2.343 7 5.25 7c1.483 0 2.798.667 3.97 1.56.967.735 1.893 1.667 2.78 2.6ZM5.25 8.5c-2.032 0-3.75 1.895-3.75 3.75S3.218 16 5.25 16c1.017 0 2.014-.457 3.062-1.253.89-.678 1.758-1.554 2.655-2.497-.897-.943-1.765-1.82-2.655-2.497C7.264 8.957 6.267 8.5 5.25 8.5Zm7.783 3.75c.897.943 1.765 1.82 2.655 2.497C16.736 15.543 17.733 16 18.75 16c2.032 0 3.75-1.895 3.75-3.75S20.782 8.5 18.75 8.5c-1.017 0-2.014.457-3.062 1.253-.89.678-1.758 1.554-2.655 2.497Z" /></svg>
   }
}