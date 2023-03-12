#[cfg(feature = "FaSolidBottleDroplet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidBottleDroplet")]
/// *This icon requires the feature* `FaSolidBottleDroplet` *to be enabled*.
#[component]
pub fn BottleDroplet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 512"><path d="M64 0C50.7 0 40 10.7 40 24s10.7 24 24 24c4.4 0 8 3.6 8 8v64.9c0 12.2-7.2 23.1-17.2 30.1C21.7 174.1 0 212.5 0 256V448c0 35.3 28.7 64 64 64H192c35.3 0 64-28.7 64-64V256c0-43.5-21.7-81.9-54.8-105c-10-7-17.2-17.9-17.2-30.1V56c0-4.4 3.6-8 8-8c13.3 0 24-10.7 24-24s-10.7-24-24-24l-8 0 0 0 0 0H72l0 0 0 0L64 0zm64 382c-26.5 0-48-20.1-48-45c0-16.8 22.1-48.1 36.3-66.4c6-7.8 17.5-7.8 23.5 0C153.9 288.9 176 320.2 176 337c0 24.9-21.5 45-48 45z" /></svg>
   }
}