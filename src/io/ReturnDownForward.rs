#[cfg(feature = "IoReturnDownForward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoReturnDownForward")]
/// *This icon requires the feature* `IoReturnDownForward` *to be enabled*.
#[component]
pub fn ReturnDownForward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="400 352 464 288 400 224" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M448,288H154C95.24,288,48,238.67,48,180V160" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}