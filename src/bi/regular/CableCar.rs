#[cfg(feature = "BiRegularCableCar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCableCar")]
/// *This icon requires the feature* `BiRegularCableCar` *to be enabled*.
#[component]
pub fn CableCar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m2 9.76 9-2.45V10H7a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-8a2 2 0 0 0-2-2h-4V6.76l9-2.45V2.24L2 7.69zM11 12v3H7v-3zm6 0v8H7v-3h10v-2h-4v-3z" /></svg>
   }
}