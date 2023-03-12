#[cfg(feature = "IoLogInOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLogInOutline")]
/// *This icon requires the feature* `IoLogInOutline` *to be enabled*.
#[component]
pub fn LogInOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M192,176V136a40,40,0,0,1,40-40H392a40,40,0,0,1,40,40V376a40,40,0,0,1-40,40H240c-22.09,0-48-17.91-48-40V336" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="288 336 368 256 288 176" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="80" y1="256" x2="352" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}