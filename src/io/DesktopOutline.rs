#[cfg(feature = "IoDesktopOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoDesktopOutline")]
/// *This icon requires the feature* `IoDesktopOutline` *to be enabled*.
#[component]
pub fn DesktopOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="32" y="64" width="448" height="320" rx="32" ry="32" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><polygon points="304 448 296 384 216 384 208 448 304 448" style="stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="368" y1="448" x2="144" y2="448" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M32,304v48a32.09,32.09,0,0,0,32,32H448a32.09,32.09,0,0,0,32-32V304Zm224,64a16,16,0,1,1,16-16A16,16,0,0,1,256,368Z" /></svg>
   }
}