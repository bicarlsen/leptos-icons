#[cfg(feature = "BiRegularFontColor")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFontColor")]
/// *This icon requires the feature* `BiRegularFontColor` *to be enabled*.
#[component]
pub fn FontColor(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 18h14v3H5zm7.5-14h-1c-.401 0-.764.24-.921.609L5.745 16h2.173l1.273-3h5.604l1.268 3h2.171L13.421 4.61A1 1 0 0 0 12.5 4zm-2.46 7 1.959-4.616L13.95 11h-3.91z" /></svg>
   }
}