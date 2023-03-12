#[cfg(feature = "FaSolidStop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidStop")]
/// *This icon requires the feature* `FaSolidStop` *to be enabled*.
#[component]
pub fn Stop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M0 128C0 92.7 28.7 64 64 64H320c35.3 0 64 28.7 64 64V384c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V128z" /></svg>
   }
}