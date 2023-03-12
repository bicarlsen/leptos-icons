#[cfg(feature = "FaSolidP")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidP")]
/// *This icon requires the feature* `FaSolidP` *to be enabled*.
#[component]
pub fn P(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M0 96C0 60.7 28.7 32 64 32h96c88.4 0 160 71.6 160 160s-71.6 160-160 160H64v96c0 17.7-14.3 32-32 32s-32-14.3-32-32V320 96zM64 288h96c53 0 96-43 96-96s-43-96-96-96H64V288z" /></svg>
   }
}