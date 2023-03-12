#[cfg(feature = "FaBrandsHouzz")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaBrandsHouzz")]
/// *This icon requires the feature* `FaBrandsHouzz` *to be enabled*.
#[component]
pub fn Houzz(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M275.9 330.7H171.3V480H17V32h109.5v104.5l305.1 85.6V480H275.9z" /></svg>
   }
}