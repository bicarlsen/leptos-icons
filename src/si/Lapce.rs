#[cfg(feature = "SiLapce")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiLapce")]
/// *This icon requires the feature* `SiLapce` *to be enabled*.
#[component]
pub fn Lapce(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M3.802 1.267 1.608 0v24L8 20.31v-2.535L3.802 20.2Zm4.208 13.9V6.231L18.003 12l-7.798 4.503v2.533L22.392 12 5.806 2.424V16.44Zm5.598-3.231L10.205 9.97v3.93Z" /></svg>
   }
}