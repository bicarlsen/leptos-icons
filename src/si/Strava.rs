#[cfg(feature = "SiStrava")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiStrava")]
/// *This icon requires the feature* `SiStrava` *to be enabled*.
#[component]
pub fn Strava(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M15.387 17.944l-2.089-4.116h-3.065L15.387 24l5.15-10.172h-3.066m-7.008-5.599l2.836 5.598h4.172L10.463 0l-7 13.828h4.169" /></svg>
   }
}