#[cfg(feature = "BiSolidCarBattery")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCarBattery")]
/// *This icon requires the feature* `BiSolidCarBattery` *to be enabled*.
#[component]
pub fn CarBattery(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 6H4c-1.103 0-2 .897-2 2v10c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V8c0-1.103-.897-2-2-2zM9 14H4v-2h5v2zm11 0h-2v2h-2v-2h-2v-2h2v-2h2v2h2v2zM4 3h4v2H4zm12 0h4v2h-4z" /></svg>
   }
}