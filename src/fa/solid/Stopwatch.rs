#[cfg(feature = "FaSolidStopwatch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidStopwatch")]
/// *This icon requires the feature* `FaSolidStopwatch` *to be enabled*.
#[component]
pub fn Stopwatch(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M160 0c-17.7 0-32 14.3-32 32s14.3 32 32 32h16V98.4C76.3 113.8 0 200 0 304C0 418.9 93.1 512 208 512s208-93.1 208-208c0-41.8-12.3-80.7-33.5-113.2l24.1-24.1c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L339.7 143c-28.1-23-62.2-38.8-99.7-44.6V64h16c17.7 0 32-14.3 32-32s-14.3-32-32-32H208 160zm72 192V320c0 13.3-10.7 24-24 24s-24-10.7-24-24V192c0-13.3 10.7-24 24-24s24 10.7 24 24z" /></svg>
   }
}