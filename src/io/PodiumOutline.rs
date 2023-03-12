#[cfg(feature = "IoPodiumOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPodiumOutline")]
/// *This icon requires the feature* `IoPodiumOutline` *to be enabled*.
#[component]
pub fn PodiumOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M32,160V456a8,8,0,0,0,8,8H176V160a16,16,0,0,0-16-16H48A16,16,0,0,0,32,160Z" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M320,48H192a16,16,0,0,0-16,16V464H336V64A16,16,0,0,0,320,48Z" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M464,208H352a16,16,0,0,0-16,16V464H472a8,8,0,0,0,8-8V224A16,16,0,0,0,464,208Z" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}