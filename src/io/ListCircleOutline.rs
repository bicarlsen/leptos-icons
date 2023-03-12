#[cfg(feature = "IoListCircleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoListCircleOutline")]
/// *This icon requires the feature* `IoListCircleOutline` *to be enabled*.
#[component]
pub fn ListCircleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><line x1="224" y1="184" x2="352" y2="184" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="224" y1="256" x2="352" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="224" y1="327" x2="352" y2="327" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M448,258c0-106-86-192-192-192S64,152,64,258s86,192,192,192S448,364,448,258Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><circle cx="168" cy="184" r="8" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="168" cy="257" r="8" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="168" cy="328" r="8" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}