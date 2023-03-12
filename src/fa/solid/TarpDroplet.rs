#[cfg(feature = "FaSolidTarpDroplet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidTarpDroplet")]
/// *This icon requires the feature* `FaSolidTarpDroplet` *to be enabled*.
#[component]
pub fn TarpDroplet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M288 160c-35.3 0-64-26.9-64-60c0-24 33.7-70.1 52.2-93.5c6.1-7.7 17.5-7.7 23.6 0C318.3 29.9 352 76 352 100c0 33.1-28.7 60-64 60zM64 128H197.5c13.2 37.3 48.7 64 90.5 64s77.4-26.7 90.5-64H512c35.3 0 64 28.7 64 64V352H448c-17.7 0-32 14.3-32 32l0 128L64 512c-35.3 0-64-28.7-64-64V192c0-35.3 28.7-64 64-64zM448 512l0-128H576L448 512zM96 256a32 32 0 1 0 0-64 32 32 0 1 0 0 64z" /></svg>
   }
}