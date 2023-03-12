#[cfg(feature = "FaSolidY")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidY")]
/// *This icon requires the feature* `FaSolidY` *to be enabled*.
#[component]
pub fn Y(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M58 45.4C47.8 31 27.8 27.7 13.4 38S-4.3 68.2 6 82.6L160 298.3V448c0 17.7 14.3 32 32 32s32-14.3 32-32V298.3L378 82.6c10.3-14.4 6.9-34.4-7.4-44.6S336.2 31 326 45.4L192 232.9 58 45.4z" /></svg>
   }
}