#[cfg(feature = "IoTvSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTvSharp")]
/// *This icon requires the feature* `IoTvSharp` *to be enabled*.
#[component]
pub fn TvSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M488,384H24a8,8,0,0,1-8-8V88a8,8,0,0,1,8-8H488a8,8,0,0,1,8,8V376A8,8,0,0,1,488,384Z" /><rect x="112" y="400" width="288" height="32" rx="4" ry="4" /></svg>
   }
}