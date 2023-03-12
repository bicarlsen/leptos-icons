#[cfg(feature = "FaSolidGripLinesVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidGripLinesVertical")]
/// *This icon requires the feature* `FaSolidGripLinesVertical` *to be enabled*.
#[component]
pub fn GripLinesVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 192 512"><path d="M64 64c0-17.7-14.3-32-32-32S0 46.3 0 64V448c0 17.7 14.3 32 32 32s32-14.3 32-32V64zm128 0c0-17.7-14.3-32-32-32s-32 14.3-32 32V448c0 17.7 14.3 32 32 32s32-14.3 32-32V64z" /></svg>
   }
}