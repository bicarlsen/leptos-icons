#[cfg(feature = "OcLgShareAndroid")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgShareAndroid")]
/// *This icon requires the feature* `OcLgShareAndroid` *to be enabled*.
#[component]
pub fn ShareAndroid(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 5.5a3.498 3.498 0 0 1-6.062 2.385l-5.112 3.021a3.498 3.498 0 0 1 0 2.188l5.112 3.021a3.5 3.5 0 1 1-.764 1.29l-5.112-3.02a3.499 3.499 0 1 1-3.843-5.642 3.499 3.499 0 0 1 3.843.872l5.112-3.021A3.5 3.5 0 1 1 20 5.5Zm-1.5 13a2 2 0 1 0-3.999-.001 2 2 0 0 0 3.999.001Zm0-13a2 2 0 1 0-3.999-.001A2 2 0 0 0 18.5 5.5ZM5.5 14a2 2 0 1 0 .001-3.999A2 2 0 0 0 5.5 14Z" /></svg>
   }
}