#[cfg(feature = "IoMailUnreadOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMailUnreadOutline")]
/// *This icon requires the feature* `IoMailUnreadOutline` *to be enabled*.
#[component]
pub fn MailUnreadOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M320,96H88a40,40,0,0,0-40,40V376a40,40,0,0,0,40,40H422.73a40,40,0,0,0,40-40V239" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="112 160 256 272 343 206.33" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="431.95" cy="128.05" r="47.95" /><path d="M432,192a63.95,63.95,0,1,1,63.95-63.95A64,64,0,0,1,432,192Zm0-95.9a32,32,0,1,0,31.95,32A32,32,0,0,0,432,96.1Z" /></svg>
   }
}