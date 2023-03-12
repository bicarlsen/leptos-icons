#[cfg(feature = "FaSolidTrailer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidTrailer")]
/// *This icon requires the feature* `FaSolidTrailer` *to be enabled*.
#[component]
pub fn Trailer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M48 32C21.5 32 0 53.5 0 80V336c0 26.5 21.5 48 48 48H65.1c7.8-54.3 54.4-96 110.9-96s103.1 41.7 110.9 96H488h8H608c17.7 0 32-14.3 32-32s-14.3-32-32-32H544V80c0-26.5-21.5-48-48-48H48zM80 96c8.8 0 16 7.2 16 16l0 131.2c-11.4 5.9-22.2 12.9-32 21V112c0-8.8 7.2-16 16-16zm96 128c-5.4 0-10.7 .2-16 .7L160 112c0-8.8 7.2-16 16-16s16 7.2 16 16l0 112.7c-5.3-.5-10.6-.7-16-.7zm80 19.2L256 112c0-8.8 7.2-16 16-16s16 7.2 16 16l0 152.2c-9.8-8.1-20.6-15.2-32-21zM368 96c8.8 0 16 7.2 16 16l0 192c0 8.8-7.2 16-16 16s-16-7.2-16-16l0-192c0-8.8 7.2-16 16-16zm112 16l0 192c0 8.8-7.2 16-16 16s-16-7.2-16-16l0-192c0-8.8 7.2-16 16-16s16 7.2 16 16zM176 480a80 80 0 1 0 0-160 80 80 0 1 0 0 160zm0-112a32 32 0 1 1 0 64 32 32 0 1 1 0-64z" /></svg>
   }
}