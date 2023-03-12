#[cfg(feature = "FaSolidStethoscope")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidStethoscope")]
/// *This icon requires the feature* `FaSolidStethoscope` *to be enabled*.
#[component]
pub fn Stethoscope(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M126.4 21.9c5.6 16.8-3.5 34.9-20.2 40.5L80 71.1V192c0 53 43 96 96 96s96-43 96-96V71.1l-26.1-8.7c-16.8-5.6-25.8-23.7-20.2-40.5s23.7-25.8 40.5-20.2l26.1 8.7C318.4 19.1 336 43.5 336 71.1V192c0 77.2-54.6 141.6-127.3 156.7C215 404.6 262.4 448 320 448c61.9 0 112-50.1 112-112V265.3c-28.3-12.3-48-40.5-48-73.3c0-44.2 35.8-80 80-80s80 35.8 80 80c0 32.8-19.7 61-48 73.3V336c0 97.2-78.8 176-176 176c-92.9 0-168.9-71.9-175.5-163.1C71.2 334.2 16 269.6 16 192V71.1c0-27.5 17.6-52 43.8-60.7L85.9 1.6c16.8-5.6 34.9 3.5 40.5 20.2zM464 224a32 32 0 1 0 0-64 32 32 0 1 0 0 64z" /></svg>
   }
}