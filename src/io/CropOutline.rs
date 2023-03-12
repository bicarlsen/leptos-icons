#[cfg(feature = "IoCropOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCropOutline")]
/// *This icon requires the feature* `IoCropOutline` *to be enabled*.
#[component]
pub fn CropOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M144,48V320a48,48,0,0,0,48,48H464" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M368,304V192a48,48,0,0,0-48-48H208" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="368" y1="368" x2="368" y2="464" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="144" y1="144" x2="48" y2="144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}