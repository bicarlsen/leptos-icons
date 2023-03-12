#[cfg(feature = "ImBinoculars")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImBinoculars")]
/// *This icon requires the feature* `ImBinoculars` *to be enabled*.
#[component]
pub fn Binoculars(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M1 0h6v1h-6zM9 0h6v1h-6zM14.875 5h-0.875v-4h-4v4h-4v-4h-4v4h-0.875c-0.619 0-1.125 0.506-1.125 1.125v8.75c0 0.619 0.506 1.125 1.125 1.125h4.75c0.619 0 1.125-0.506 1.125-1.125v-5.875h2v5.875c0 0.619 0.506 1.125 1.125 1.125h4.75c0.619 0 1.125-0.506 1.125-1.125v-8.75c0-0.619-0.506-1.125-1.125-1.125zM5.438 15h-3.875c-0.309 0-0.563-0.225-0.563-0.5s0.253-0.5 0.563-0.5h3.875c0.309 0 0.563 0.225 0.563 0.5s-0.253 0.5-0.563 0.5zM8.5 8h-1c-0.275 0-0.5-0.225-0.5-0.5s0.225-0.5 0.5-0.5h1c0.275 0 0.5 0.225 0.5 0.5s-0.225 0.5-0.5 0.5zM14.438 15h-3.875c-0.309 0-0.563-0.225-0.563-0.5s0.253-0.5 0.563-0.5h3.875c0.309 0 0.563 0.225 0.563 0.5s-0.253 0.5-0.563 0.5z" /></svg>
   }
}