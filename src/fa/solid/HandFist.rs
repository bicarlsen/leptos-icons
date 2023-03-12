#[cfg(feature = "FaSolidHandFist")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidHandFist")]
/// *This icon requires the feature* `FaSolidHandFist` *to be enabled*.
#[component]
pub fn HandFist(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M160 0c17.7 0 32 14.3 32 32V144H128V32c0-17.7 14.3-32 32-32zM32 64c0-17.7 14.3-32 32-32s32 14.3 32 32v80H32V64zm192 0c0-17.7 14.3-32 32-32s32 14.3 32 32v96c0 17.7-14.3 32-32 32s-32-14.3-32-32V64zm96 64c0-17.7 14.3-32 32-32s32 14.3 32 32v64c0 17.7-14.3 32-32 32s-32-14.3-32-32V128zm-96 88l0-.6c9.4 5.4 20.3 8.6 32 8.6c13.2 0 25.4-4 35.6-10.8c8.7 24.9 32.5 42.8 60.4 42.8c11.7 0 22.6-3.1 32-8.6V256c0 52.3-25.1 98.8-64 128v96c0 17.7-14.3 32-32 32H128c-17.7 0-32-14.3-32-32V401.6c-17.3-7.9-33.2-18.8-46.9-32.5L37.5 357.5C13.5 333.5 0 300.9 0 267V240c0-35.3 28.7-64 64-64h88c22.1 0 40 17.9 40 40s-17.9 40-40 40H96c-8.8 0-16 7.2-16 16s7.2 16 16 16h56c39.8 0 72-32.2 72-72z" /></svg>
   }
}