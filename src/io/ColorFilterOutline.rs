#[cfg(feature = "IoColorFilterOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoColorFilterOutline")]
/// *This icon requires the feature* `IoColorFilterOutline` *to be enabled*.
#[component]
pub fn ColorFilterOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="184" r="120" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><circle cx="344" cy="328" r="120" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><circle cx="168" cy="328" r="120" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}