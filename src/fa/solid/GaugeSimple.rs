#[cfg(feature = "FaSolidGaugeSimple")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidGaugeSimple")]
/// *This icon requires the feature* `FaSolidGaugeSimple` *to be enabled*.
#[component]
pub fn GaugeSimple(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256zm320 96c0-26.9-16.5-49.9-40-59.3V88c0-13.3-10.7-24-24-24s-24 10.7-24 24V292.7c-23.5 9.5-40 32.5-40 59.3c0 35.3 28.7 64 64 64s64-28.7 64-64z" /></svg>
   }
}