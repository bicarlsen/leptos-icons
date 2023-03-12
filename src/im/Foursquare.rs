#[cfg(feature = "ImFoursquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFoursquare")]
/// *This icon requires the feature* `ImFoursquare` *to be enabled*.
#[component]
pub fn Foursquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M13.306 1.408c-0.188-0.256-0.488-0.408-0.806-0.408h-9.5c-0.552 0-1 0.448-1 1v12c0 0.404 0.244 0.769 0.617 0.924 0.124 0.051 0.254 0.076 0.382 0.076 0.26 0 0.516-0.102 0.707-0.293l3.707-3.707h2.586c0.437 0 0.824-0.284 0.954-0.702l2.5-8c0.095-0.304 0.040-0.634-0.149-0.891zM10.515 5h-3.515c-0.552 0-1 0.448-1 1s0.448 1 1 1h2.89l-0.625 2h-2.265c-0.265 0-0.52 0.105-0.707 0.293l-2.293 2.293v-8.586h7.14l-0.625 2z" /></svg>
   }
}