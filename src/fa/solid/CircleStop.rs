#[cfg(feature = "FaSolidCircleStop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidCircleStop")]
/// *This icon requires the feature* `FaSolidCircleStop` *to be enabled*.
#[component]
pub fn CircleStop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM192 160H320c17.7 0 32 14.3 32 32V320c0 17.7-14.3 32-32 32H192c-17.7 0-32-14.3-32-32V192c0-17.7 14.3-32 32-32z" /></svg>
   }
}