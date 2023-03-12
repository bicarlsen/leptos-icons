#[cfg(feature = "IoReturnDownBack")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoReturnDownBack")]
/// *This icon requires the feature* `IoReturnDownBack` *to be enabled*.
#[component]
pub fn ReturnDownBack(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="112 352 48 288 112 224" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M64,288H358c58.76,0,106-49.33,106-108V160" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}