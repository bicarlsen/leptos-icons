#[cfg(feature = "IoCloudyOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCloudyOutline")]
/// *This icon requires the feature* `IoCloudyOutline` *to be enabled*.
#[component]
pub fn CloudyOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M100.18,241.19a15.93,15.93,0,0,0,13.37-13.25C126.6,145.59,186.34,96,256,96c64.69,0,107.79,42.36,124.92,87a16.11,16.11,0,0,0,12.53,10.18C449.36,202.06,496,239.21,496,304c0,66-54,112-120,112H116c-55,0-100-27.44-100-88C16,273.57,59.89,247.19,100.18,241.19Z" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}