#[cfg(feature = "FaSolidRadiation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidRadiation")]
/// *This icon requires the feature* `FaSolidRadiation` *to be enabled*.
#[component]
pub fn Radiation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M215 186.7c-23.9 13.8-40 39.7-40 69.3L31 256c-17.7 0-32.2-14.4-30-31.9C9.7 154 46.8 92.7 100.3 52c14.1-10.7 33.8-5.3 42.7 10l72 124.7zM255 336c14.6 0 28.2-3.9 40-10.7l72 124.8c8.8 15.3 3.7 35.1-12.6 41.9c-30.6 12.9-64.2 20-99.4 20s-68.9-7.1-99.4-20c-16.3-6.9-21.4-26.6-12.6-41.9l72-124.8c11.8 6.8 25.4 10.7 40 10.7zm224-80l-144 0c0-29.6-16.1-55.5-40-69.3L367 62c8.8-15.3 28.6-20.7 42.7-10c53.6 40.7 90.6 102 99.4 172.1c2.2 17.5-12.4 31.9-30 31.9zM257 208a48 48 0 1 1 -2 96 48 48 0 1 1 2-96z" /></svg>
   }
}