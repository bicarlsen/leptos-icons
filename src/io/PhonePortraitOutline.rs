#[cfg(feature = "IoPhonePortraitOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPhonePortraitOutline")]
/// *This icon requires the feature* `IoPhonePortraitOutline` *to be enabled*.
#[component]
pub fn PhonePortraitOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="128" y="16" width="256" height="480" rx="48" ry="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M176,16h24a8,8,0,0,1,8,8h0a16,16,0,0,0,16,16h64a16,16,0,0,0,16-16h0a8,8,0,0,1,8-8h24" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}