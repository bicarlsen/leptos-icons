#[cfg(feature = "IoVolumeLowOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoVolumeLowOutline")]
/// *This icon requires the feature* `IoVolumeLowOutline` *to be enabled*.
#[component]
pub fn VolumeLowOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M189.65,192H120a8,8,0,0,0-8,8V312a8,8,0,0,0,8,8h69.65a16,16,0,0,1,10.14,3.63l91.47,75A8,8,0,0,0,304,392.17V119.83a8,8,0,0,0-12.74-6.44l-91.47,75A16,16,0,0,1,189.65,192Z" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M384,320c9.74-19.41,16-40.81,16-64,0-23.51-6-44.4-16-64" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}