#[cfg(feature = "IoCropSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCropSharp")]
/// *This icon requires the feature* `IoCropSharp` *to be enabled*.
#[component]
pub fn CropSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="166 346 166 32 122 32 122 122 32 122 32 166 122 166 122 390 346 390 346 480 390 480 390 390 480 390 480 346 166 346" /><polygon points="346 320 390 320 390 122 192 122 192 166 346 166 346 320" /></svg>
   }
}