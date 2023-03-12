#[cfg(feature = "IoBrowsersOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBrowsersOutline")]
/// *This icon requires the feature* `IoBrowsersOutline` *to be enabled*.
#[component]
pub fn BrowsersOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="48" y="64" width="416" height="384" rx="48" ry="48" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><path d="M397.82,64H114.18C77.69,64,48,94.15,48,131.2V176H64c0-16,16-32,32-32H416c16,0,32,16,32,32h16V131.2C464,94.15,434.31,64,397.82,64Z" /></svg>
   }
}