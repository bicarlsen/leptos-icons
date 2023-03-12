#[cfg(feature = "IoCheckboxOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCheckboxOutline")]
/// *This icon requires the feature* `IoCheckboxOutline` *to be enabled*.
#[component]
pub fn CheckboxOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="352 176 217.6 336 160 272" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><rect x="64" y="64" width="384" height="384" rx="48" ry="48" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}